extern crate diesel;
extern crate chrono;
use diesel::SqliteConnection;
use chan::models;
use chan::chan_service;
use chan::schema::threads::dsl::*;
use diesel::prelude::*;
use self::chrono::prelude::*;
use chan::schema::replies as reply_dsl;
use chan::schema::items::dsl::*;

#[derive(Debug)]
pub enum action_error {
	insert_error,
	update_error,
	get_error
}


pub struct DBThreadModelAction
{
}

pub struct DBThreadReplyModelAction
{

}

pub struct DBThreadItemsModelAction
{

}
#[derive(Debug)]
pub enum action_success {
	success,
}
pub trait model_actions<'a>
{
	type model_item;
	type chan_item;
	fn create(&self,to_create:&'a Self::chan_item) -> Self::model_item;
	fn insert(&self,to_insert:&'a Self::chan_item,connection : &'a SqliteConnection)-> Result<action_success,action_error>;
	fn update(&self,to_update:&'a Self::chan_item,connection : &'a SqliteConnection)-> Result<action_success,action_error>;
	fn get_by(&self,id:i32,connection : &'a SqliteConnection)-> Option<Self::chan_item>;

}

impl<'a> model_actions<'a> for DBThreadItemsModelAction
{
	type model_item  = models::DBItem;
	type chan_item = chan_service::Item;

	fn create(&self,to_create:&'a Self::chan_item) -> Self::model_item
	{
		models::DBItem
		{
			item_id : to_create.item_id,
			key : to_create.key.clone(),
			value : to_create.value.clone(),
			thread : to_create.thread,
		}
	}

	fn insert(&self,to_insert:&'a Self::chan_item,connection : &'a SqliteConnection)-> Result<action_success,action_error>
	{
		let new_item = models::NewDBItem{
			key : to_insert.key.clone(),
			value : to_insert.value.clone(),
			thread : to_insert.thread
		};
		let result = diesel::insert(&new_item)
		         .into(items)
		         .execute(connection);
		match result 
		{
		    Ok(good) => Ok(action_success::success),
		  Err(_) => Err(action_error::insert_error)
		}
	}

	fn update(&self,to_update:&'a Self::chan_item,connection : &'a SqliteConnection)-> Result<action_success,action_error>
	{
		let existing_item = self.create(to_update);
		match diesel::update(items.find(to_update.item_id))
		.set(&existing_item)
		.execute(connection) 
		{	
		  Ok(good) => Ok(action_success::success),
		  Err(_) => Err(action_error::update_error)
		}
	}

	fn get_by(&self,t_id:i32,connection : &'a SqliteConnection)-> Option<Self::chan_item>
	{
		let results = items
	              .filter(item_id.eq(t_id))
	              .load::<models::DBItem>(connection);
	match results
	{
		Ok(unwrapped_threads) =>
		{
			match unwrapped_threads.first()
			{
				Some(first) =>Some(chan_service::Item
				{
					item_id : first.item_id,
					key: first.key.clone(),
					value : first.value.clone(),
					thread : first.thread
					
				}),
				None => None
			}


		},
		Err(_) =>None
	}
	}
}

impl<'a> model_actions<'a> for DBThreadModelAction
{
	type model_item = models::DBThread;
	type chan_item = chan_service::Thread;
	fn create(&self,to_create:&'a Self::chan_item) -> Self::model_item
	{
		let date_time_literal = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
		let comment = match to_create.com{Some(ref thread_com)=> thread_com.clone(), None=>String::from("")};
		models::DBThread
		{
			thread_id : match to_create.no{Some(thread_number)=>thread_number,None=>-1},
			operation_name : get_operation_name(comment.split_whitespace().map(|s|{String::from(s)}).collect::<Vec<String>>()),
			operation_text : comment,
			replies : match to_create.replies {Some(thread_replies)=>thread_replies,None=>-1},
			last_updated : date_time_literal,
		}
	}


	fn insert(&self,to_insert:&'a Self::chan_item,connection : &'a SqliteConnection)-> Result<action_success,action_error>
	{
		let new_thread = self.create(to_insert);
		let result = diesel::insert(&new_thread)
		         .into(threads)
		         .execute(connection);
		match result 
		{
		    Ok(good) => Ok(action_success::success),
		  Err(_) => Err(action_error::insert_error)
		}
	}

	fn update(&self,to_update:&'a Self::chan_item,connection : &'a SqliteConnection)-> Result<action_success,action_error>
	{
		let existing_thread = self.create(to_update);
		match diesel::update(threads.find(existing_thread.thread_id))
		.set(&existing_thread)
		.execute(connection) 
		{	
		  Ok(good) => Ok(action_success::success),
		  Err(_) => Err(action_error::update_error)
		}
	}

	fn get_by(&self,t_id:i32,connection : &'a SqliteConnection)-> Option<Self::chan_item>
	{
		let results = threads
	              .filter(thread_id.eq(t_id))
	              .load::<Self::model_item>(connection);
	match results
	{
		Ok(unwrapped_threads) =>
		{
			match unwrapped_threads.first()
			{
				Some(first) =>Some(chan_service::Thread
				{
					no : Some(first.thread_id),
					now : None,
					name : None,
					com : None,
					ext : None,
					w : None,
					h : None,
					tn_w : None,
					tn_h : None,
					tim : None,
					time : None,
					md5 : None,
					fsize : None,
					resto : None,
					bumplimit : None,
					imagelimit : None,
					semantic_url : None,
					custom_spoiler : None,
					replies : Some(first.replies),
					omitted_posts : None,
					omitted_images : None,
					last_replies : None,
					last_modified : None,
				}),
				None => None
			}


		},
		Err(_) =>None
	}
	}


}

impl<'a> model_actions<'a> for DBThreadReplyModelAction
{
	type model_item = models::DBThreadReply;
	type chan_item= chan_service::Post;

	fn create(&self,to_create:&'a Self::chan_item) -> Self::model_item
	{
		let date_time_literal = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
		let comment = match to_create.com{Some(ref thread_com)=> thread_com.clone(), None=>String::from("")};
		models::DBThreadReply
		{
			reply_id : match to_create.no{Some(thread_number)=>thread_number,None=>-1},
			reply_comment : comment,
			last_updated : date_time_literal,
			thread : match to_create.no {Some(thread_replies)=>thread_replies,None=>-1},
		}
	}
	fn insert(&self,to_insert:&'a Self::chan_item,connection : &'a SqliteConnection)-> Result<action_success,action_error>
	{
		let new_reply = self.create(to_insert);
		let result = diesel::insert(&new_reply)
		     .into(reply_dsl::table)
		     .execute(connection);
		match result 
		{
		    Ok(good) => Ok(action_success::success),
		    Err(_) => Err(action_error::insert_error)
		}
	}
	fn update(&self,to_update:&'a Self::chan_item,connection : &'a SqliteConnection)-> Result<action_success,action_error>
	{
		let existing_reply = self.create(to_update);
		match diesel::update(reply_dsl::dsl::replies.find(existing_reply.reply_id))
		      .set(&existing_reply)
		      .execute(connection) 
		{
			 Ok(good) => Ok(action_success::success),
		    Err(_) => Err(action_error::update_error)
		}
	}
	fn get_by(&self,t_id:i32,connection : &'a SqliteConnection)-> Option<Self::chan_item>
	{
		let results = reply_dsl::table
	              .filter(reply_dsl::dsl::reply_id.eq(t_id))
	              .load::<models::DBThreadReply>(connection);
		match results
		{
			Ok(unwrapped_threads) =>
			{
				match unwrapped_threads.first()
				{
					Some(first) =>Some(chan_service::Post
					{
						no : Some(t_id),
						resto : None,
						sticky : None,
						closed : None,
						archived : None,
						archived_on : None,
						now : None,
						time : None,
						name : None,
						trip : None,
						id : None,
						capcode : None,
						country : None,
						country_name : None,
						sub : None,
						com : Some(first.reply_comment.clone()),
						tim : None,
						filename : None,
						ext : None,
						fsize : None,
						md5 : None,
						w : None,
						h : None,
						tn_w : None,
						tn_h : None,
						filedeleted : None,
						spoiler : None,
						custom_spoiler : None,
						omitted_posts : None,
						omitted_images : None,
						replies : None,
						images : None,
						bumplimit : None,
						imagelimit : None,
						last_modified : None,
						tag : None,
						semantic_url : None,
						since4pass : None,
						thread_no : Some(first.thread as u64)

					}),
					None => None
				}


			},
			Err(_) =>None
		}
	}


}



fn get_operation_name(mut word_array : Vec<String>) -> String
{
     let first_word = match word_array.first()
     {
     	Some(fw) => fw.clone(),
     	None =>String::from("")
     };
     match first_word.to_uppercase().contains("OPERATION")
     {
     	true =>
     	{
     		word_array.truncate(2);
     		word_array.join(" ")

     	} 
     	_ =>
     	{
     		if(word_array.len()>0)
     		{
     			get_operation_name(word_array.clone().split_off(1))
     		}
     		else 
     		{
     			String::from("")
     		}
     	}
     }
}

#[test]
fn test_operation_name()
{
	assert_eq!(get_operation_name(vec![String::from("check"),String::from("Operation"),String::from("me")]),"Operation me");
	assert_eq!(get_operation_name(vec![String::from("check"),String::from("operation"),String::from("me")]),"operation me");
	assert_eq!(get_operation_name(vec![String::from("OPERATION"),String::from("me")]),"OPERATION me");
	assert_eq!(get_operation_name(vec![String::from("check"),String::from("ation"),String::from("m"),String::from("hi")]),"");
	assert_eq!(get_operation_name(vec![]),"");

}