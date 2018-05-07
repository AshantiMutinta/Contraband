use chan::schema::*;
#[derive(Insertable, Queryable,Associations,AsChangeset)]
#[table_name = "threads"]
pub struct DBThread
{
	pub thread_id: i32,
	pub operation_name :String,
	pub operation_text : String,
	pub replies : i32,
	pub last_updated : String
}

#[derive(Insertable, Queryable,Associations,AsChangeset)]
#[table_name = "replies"]
pub struct DBThreadReply
{
	pub reply_id :i32,
	pub reply_comment : String,
	pub last_updated : String,
	pub thread: i32
}

#[derive(Insertable, Queryable,Associations,AsChangeset)]
#[table_name = "items"]
pub struct DBItem
{
	pub item_id : i32,
	pub key : String,
	pub value : String,
	pub thread: i32
}

#[derive(Insertable, Queryable,Associations,AsChangeset)]
#[table_name = "items"]
pub struct NewDBItem
{
	pub key : String,
	pub value : String,
	pub thread: i32
}



