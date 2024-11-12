use minidom::Element;
use std::fs::File;
use std::io::Read;
use std::error::Error;

fn explore_root(file_name: &str) -> Result<(), Box<dyn Error>> {
    // Step 1: Load the XML file content into a string
    let mut contents = String::new();
    File::open(file_name)?.read_to_string(&mut contents)?;

    // Step 2: Parse the content into an XML root element
    let root: Element = contents.parse()?;

    // Step 3: Print the name and namespace of the root element
    println!("Root element: {}", root.name());

    // Step 3.1: Print the attributes of the root element
    println!("Root element attributes:");
    for (name, value) in root.attrs() {
        println!("New attribute found in root: {} = {}", name, value);
    }

    // Step 4: Print the top level elements found in the root

    println!("Top level elements found in the root:");
    for child in root.children() {
        println!("/nElement:  {}", child.name());
        for (name, value) in child.attrs() {
            println!("/tNew attribute found for {}: {} = {}", child.name(), name, value);
        }
    }


    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    explore_root("test_data/T113_0.xml")?;
    Ok(())
}