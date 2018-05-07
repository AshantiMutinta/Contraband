use reqwest;
use serde::de::DeserializeOwned;

#[derive(Clone)]
pub struct Item
{
	pub item_id : i32,
	pub key : String,
	pub value : String,
	pub thread : i32
}


#[derive(Deserialize, Serialize,Clone,Debug)]
pub struct LastReplies
{
	pub no : Option<u64>,
	pub now : Option<String>,
	pub name : Option<String>,
	pub com : Option<String>,
	pub time : Option<u64>,
	pub resto : Option<u64>,
}

#[derive(Deserialize, Serialize,Clone,Debug)]
pub struct Thread
{
	pub no : Option<i32>,
	pub now : Option<String>,
	pub name : Option<String>,
	pub com : Option<String>,
	pub ext : Option<String>,
	pub w : Option<i32>,
	pub h : Option<i32>,
	pub tn_w : Option<u32>,
	pub tn_h : Option<u32>,
	pub tim : Option<u64>,
	pub time : Option<u64>,
	pub md5 : Option<String>,
	pub fsize : Option<u32>,
	pub resto : Option<u32>,
	pub bumplimit : Option<u32>,
	pub imagelimit : Option<u32>,
	pub semantic_url : Option<String>,
	pub custom_spoiler : Option<u8>,
	pub replies : Option<i32>,
	pub omitted_posts : Option<u16>,
	pub omitted_images : Option<u16>,
	pub last_replies : Option<Vec<LastReplies>>,
	pub last_modified : Option<u64>,
}

#[derive(Deserialize, Serialize,Clone,Debug)]
pub struct PageThread
{

   pub page : Option<u64>,
   pub threads : Option<Vec<Thread>>
}

#[derive(Deserialize, Serialize,Clone,Debug)]
pub struct Posts 
{
	pub posts : Vec<Post>
} 

#[derive(Deserialize, Serialize,Clone,Debug)]
pub struct Post
{
	pub no : Option<i32>,
	pub resto : Option<u64>,
	pub sticky : Option<u64>,
	pub closed : Option<u64>,
	pub archived : Option<u64>,
	pub archived_on : Option<u64>,
	pub now : Option<String>,
	pub time : Option<u64>,
	pub name : Option<String>,
	pub trip : Option<String>,
	pub id : Option<String>,
	pub capcode : Option<String>,
	pub country : Option<String>,
	pub country_name : Option<String>,
	pub sub : Option<String>,
	pub com : Option<String>,
	pub tim : Option<u64>,
	pub filename : Option<String>,
	pub ext : Option<String>,
	pub fsize : Option<u64>,
	pub md5 : Option<String>,
	pub w : Option<u64>,
	pub h : Option<u64>,
	pub tn_w : Option<u64>,
	pub tn_h : Option<u64>,
	pub filedeleted : Option<u64>,
	pub spoiler : Option<u64>,
	pub custom_spoiler : Option<u64>,
	pub omitted_posts : Option<u64>,
	pub omitted_images : Option<u64>,
	pub replies : Option<u64>,
	pub images : Option<u64>,
	pub bumplimit : Option<u64>,
	pub imagelimit : Option<u64>,
	pub last_modified : Option<u64>,
	pub tag : Option<String>,
	pub semantic_url : Option<String>,
	pub since4pass : Option<u64>,
	pub thread_no : Option<u64>

}

fn get<T> (uri : &str,on_error_function: &Fn() -> T ) -> T where T: DeserializeOwned
{
	let chan_client = reqwest::Client::new();
     match chan_client.get(uri).send()
     {
     	Ok(mut resp) =>
     	{
			match resp.status().is_success()
			{
				true =>
				{
					match resp.json::<T>()
					{
						Ok(catalog) =>catalog,
						Err(error)=>
						{
							eprintln!("failed to serialize {:?}",error);

							on_error_function()

						}
					}

				},
				_ =>
				{
					println!("failed to respond due to this error {:?}", resp.status());
				    on_error_function()
				}
			}

     	},
     	Err(err) =>
     	{
     		eprintln!("failed to send get {:?}",err);
     		on_error_function()

     	} 
     }
}


pub fn get_thread_replies(board:&str,thread_id : &str) -> Posts
{
	let thread_api = vec!["http://a.4cdn.org/",board,"/thread/",thread_id,".json"].join("");
	get::<Posts>(&thread_api,&||Posts{posts:vec![]})
}

pub fn get_all_operation_threads(board: &String) ->Vec<PageThread>
{
	get::<Vec<PageThread>>(&vec!["http://a.4cdn.org/",board,"/","catalog.json"].join(""),&||vec![])
}

pub fn filter_thread_by_keywords(threads:Vec<Thread>) ->Vec<Thread>
{

     threads.into_iter().filter(|thread|{
     	match thread.com
     	{
     		Some(ref comment) => comment.to_uppercase().contains("OPERATION"),
     		None => false
     	}
     }).collect::<Vec<_>>()
}