// SHAW application
// check for patterns in how the xml is parsed in the cps / qps connector
// ie. if tag name maps to key and text maps to value in json return object, then you can ignore the attributes etc
// it maybe easier to use quick xml to create structs with the event stream based on response needed

// Options
// A - using minidom
// translate xml => rs structs (generic)
// translate rs structs => json (match json response)
// B - using quick-xml
// translate xml => json (match json response)

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
	fn minidom() {
		const DATA: &'static str = r#"<articles xmlns="all-articles">
		    <article>
		        <title>title of first article</title>
		        <body>
		            Rust fixed them all. &lt;3
		        </body>
		    </article>
		    <article>
		        <title>title of second article</title>
		        <body>
		            Just kidding!
		        </body>
		    </article>
		</articles>"#;
		
        // let root: minidom::Element = string.parse().unwrap();
        let root: minidom::Element = DATA.parse().unwrap();
        // to see print stmts use the following command
        // cargo test -- --nocapture
        println!("{:#?}", root);

        println!("{:#?}", root.name());

        // println!("{}", root.children()[1].name());
	}
}
