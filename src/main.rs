extern crate FindMyOperations;
extern crate slack_hook;
extern crate diesel;
extern crate chrono;
extern crate url;

use FindMyOperations::chan::{chan_service,db_connection,models,model_actions};
use FindMyOperations::chan::schema::threads::dsl::*;
use FindMyOperations::chan::schema::replies as reply_dsl;
use slack_hook::{Slack, PayloadBuilder};
use diesel::{SqliteConnection,LoadDsl,FilterDsl,ExpressionMethods};
use diesel::prelude::*;

use chrono::prelude::*;

use std::ascii::AsciiExt;

use std::env;

use url::{Url, ParseError};
fn main() {
	let connection = db_connection::establish_connection();
    let pol_threads = chan_service::get_all_operation_threads(&String::from("pol"));
    pol_threads.into_iter().map(|p_t|
    {
		match p_t.threads 
		{

			Some(chan_threads) =>{
				let new_threads = chan_service::filter_thread_by_keywords(chan_threads);
				new_threads.iter().map(|t|
				{
				   let thread_model  = Box::new(model_actions::DBThreadModelAction{}) 
				                       as 
				                       Box<model_actions::model_actions<chan_item = chan_service::Thread,model_item = models::DBThread>>;
				   let thread_number = match t.no{Some(num)=>num,None=>-1};
				   let thread_found = thread_model.get_by(thread_number,&connection);
				   match thread_found
				   {
				   	    Some(unwraped_thread) =>
				   	    {
							if unwraped_thread.replies < t.replies
							{
								push_thread(t,&connection,&|t,connection|
								{
									let t_model  = Box::new(model_actions::DBThreadModelAction{}) 
				                       as 
				                       Box<model_actions::model_actions<chan_item = chan_service::Thread,model_item = models::DBThread>>;
									t_model.update(t,&connection);
								})
							}
							else 
							{
								();
							}
				   	    },
				   	    None =>
				   	    {
				   	    	push_thread(t,&connection,&|t,connection|
				   	    	{
				   	    		let t_model  = Box::new(model_actions::DBThreadModelAction{}) 
				                       as 
				                       Box<model_actions::model_actions<chan_item = chan_service::Thread,model_item = models::DBThread>>;
									t_model.insert(t,&connection);
								})
				   	    }
				   }
				   
				   
				}).collect::<Vec<_>>()
			},
			None =>vec![]
		};

    }).collect::<Vec<_>>();
    

}

fn does_string_have_hashtag(word:&str) -> bool
{
	let string_to_check = String::from(word);
	let mut characters = string_to_check.chars();
	match characters.next()
	{
		Some(first_character) =>{first_character=='#' && 
		                         match characters.next()
		                         {
		                         	Some(second_char) => second_char.is_alphabetic(),
		                         	None => false
		                         }
		                     },
		None => false
	}
}

fn is_string_url(url:&str) -> bool
{
	if url.contains("http")
	{
		match Url::parse(url)
		{
			Ok(_) => true,
			Err(_) => false
		}
	}
	else 
	{
	    false	
	}

}

#[test]
fn test_if_string_has_hashtag()
{
	assert_eq!(does_string_have_hashtag("#doesThisHave"),true);
	assert_eq!(does_string_have_hashtag("#12"), false);
    assert_eq!(does_string_have_hashtag("ndiocnsd#csidcn"),false);
}

fn push_thread<'a>(thread_to_push:&'a chan_service::Thread,connection : &'a SqliteConnection,on_push: &'a Fn(&'a chan_service::Thread,&'a SqliteConnection))
{
	send_thread_to_slack_channel(thread_to_push);
	on_push(thread_to_push,&connection);
	archive_all_replies(thread_to_push,&connection);

	match thread_to_push.com 
	{
		Some(ref comment) => 
		{
			match thread_to_push.no
			{
				Some(number) =>
				{

					comment.split_whitespace().map(|w|
					{
						let mut item = chan_service::Item
										{
									    item_id :0,
										key : String::from(""),
										value : String::from(w),
										thread : number
										};
						let item_model  = Box::new(model_actions::DBThreadItemsModelAction{}) 
							as 
							Box<model_actions::model_actions<chan_item = chan_service::Item,model_item = models::DBItem>>;
						if does_string_have_hashtag(w)
						{
							item.key = String::from("TWITTER-HASH");
							item_model.insert(&item,connection);

						}
						else if is_string_url(w)
						{
							item.key = String::from("URL");
							item_model.insert(&item,connection);
						}
						else 
						{
							();
						}
					}).collect::<Vec<_>>();


				},
				None =>{();}
			}

		},
		None =>{();}
	};
	

}



fn upsert_reply_into_db<'a>(model_action : Box<model_actions::model_actions<'a,model_item=models::DBThreadReply,chan_item=chan_service::Post>>, post_to_add : &'a chan_service::Post, connection: &'a SqliteConnection) 
{
	match post_to_add.no
	{
		Some(num) =>
		{
			match model_action.get_by(num,connection)
			{
				Some(first) =>
				{
				    model_action.update(post_to_add,connection);
				},
				None => {model_action.insert(post_to_add,connection);}
			}

		},
		None => {();}
	}


}

fn archive_all_replies<'a>(thread_to_save : &chan_service::Thread,connection: &'a SqliteConnection)
{ 

    match thread_to_save.no
	{
		Some(num) => 
		{
				let mut post_replies  =  chan_service::get_thread_replies("pol",&num.to_string()).posts;
				match thread_to_save.replies
				{
					Some(rep) => 
					{
						if rep >175
						{
							post_replies.iter_mut().map(|p|
							{
							     p.thread_no  = Some(match thread_to_save.no{Some(i)=>i as u64,None=>0 as u64});	
							     upsert_reply_into_db(Box::new(model_actions::DBThreadReplyModelAction{}),p,connection);
							}).collect::<Vec<_>>();
						}
						else 
						{
						    ();
						}
					},
					None => {();}
				}

		},
		None => {();}
	};
	
	
}

fn format_for_slack<T>(prompt_thread: &str,value : Option<T>,error_message:&str) -> Vec<String> where T : ToString 
{
	let mut formatted_text_buffer : Vec<String> = vec![];
	formatted_text_buffer.push(String::from(prompt_thread));
	formatted_text_buffer.push(String::from(" : ["));
	match value
	{
		Some(unwraped_value) =>{
			formatted_text_buffer.push(unwraped_value.to_string())
		},
		None => formatted_text_buffer.push(String::from(error_message))
	};
	formatted_text_buffer.push(String::from("] "));
	formatted_text_buffer
}



fn send_thread_to_slack_channel(thread_to_post : &chan_service::Thread)
{
	let mut formatted_text_buffer : Vec<String> = format_for_slack::<i32>("POSTING THREAD WITH ID", thread_to_post.no,"UNKNOWN ID");
	formatted_text_buffer.append(&mut format_for_slack::<String>("BY USER",thread_to_post.name.clone(),"UNKOWN NAME"));
	formatted_text_buffer.append(&mut format_for_slack::<String>("WITH COMMENT", thread_to_post.com.clone(),"INVALID COMMENT"));
	formatted_text_buffer.append(&mut format_for_slack::<i32>("NUMBER OF REPLIES AND COUNTING", thread_to_post.replies,"N\\A"));

	match env::var("POLBOT_SLACKHOOK")
	{
		Ok(hook_env) =>
		{

            // shouldn't continue if payload cannot be created
			let payload_result = PayloadBuilder::new()
			              .text(formatted_text_buffer.join(""))
			              .channel("pol_operations")
			              .username("polbot")
			              .build();

			match payload_result
			{
				Ok(payload) =>
				{
					let slack_client_result = Slack::new(&*hook_env);
					match slack_client_result
					{
						Ok(slack_client) => 
						{
							match slack_client.send(&payload)
							{
								Ok(_) =>println!("SENT TO SLACK"),
								Err(_) =>println!("COULD NOT SEND TO SLACK CHANNEL")

							};

						},
						Err(_) => println!("COULD NOT CREATE SLACK CLIENT")
					};
					
				},
				Err(_) =>println!("COULD NOT CREATE SLACK PAYLOAD")
			};


		},
		Err(_) =>println!("POLBOT_SLACKHOOK IS NOT SET")
	}



}


