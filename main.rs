/*#[derive(Debug)]
enum Book{
    author(String),
    name(String)
}
impl Book{
    fn sat(saturday:Book){
        //saturday.author,saturday.name
    }
pub trait BookInformation {
    fn info(self)->String;
}
impl BookInformation for Book{
    fn info(self)->String{
       format!("{},{}", self.name,self.author)
    }
    }

}
fn main(){
    let result=Book::author(String::from("Khalid Hosseni"));
    let result1=Book::name(String::from("The Kite Runner"));
   result.info(result1) 

}
*/
#[derive(Debug)]
struct Book{
    author:String,
    name:String
}
impl Book{
    fn book(self)->String{
        format!("{},{}",self.author,self.name)
    }
}
impl Book{
    //fn took(author:Book,name:Book)->String{
         //format!("{:?},{:?}", author,name)
    }
}
fn main(){
    let result1=Book{
        author:String::from("Khalid Hosseni"),
        name:String::from("The Kite Runner")
    };
println!("{}",result1.book());

let result2=Book::took("Khalid Hosseni".to_string(),"The Kite Runner".to_string());
}