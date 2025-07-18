use serde::Deserialize;

pub fn test(name: &str) -> String{
    format!("Hello, {}!", name)
}

#[derive(Deserialize)]
struct Post {
    userId: i32,
    id: i32,
    title: String,
    body: String,
}

pub async fn test_reqwest() {
    let post_url = "https://jsonplaceholder.typicode.com/posts";
    let resp = reqwest::get(post_url).await.unwrap();

    let posts: Vec<Post> = resp.json().await.unwrap();
    for post in posts {
        println!("{:?} {:?}", post.id , post.title);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test(){
        let result = test("test");
        assert_eq!(result, "Hello, test!");
    }
}