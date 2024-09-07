use std::fs::File;
use std::io::Read;
use quick_xml::events::Event;
use quick_xml::name::QName;
use quick_xml::reader::Reader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_name  = "test_data/T113_0.xml";

    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("File: {}", file_name);
    //println!("Contents: {}", contents);

    let mut reader = Reader::from_str(&contents);

    let mut folio = String::new();
    let mut fecha = String::new();
    let mut moneda = String::new();
    let mut total = String::new();
    let mut sello = String::new();
    let mut no_certificado = String::new();

    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) => {
                if e.name() == QName(b"cfdi:Comprobante") {
                    for attr in e.attributes() {
                        if let Ok(attr) = attr {
                            match attr.key {
                                QName(b"Folio") => folio = String::from_utf8_lossy(&attr.value).into_owned(),
                                QName(b"Fecha") => fecha = String::from_utf8_lossy(&attr.value).into_owned(),
                                QName(b"Moneda") => moneda = String::from_utf8_lossy(&attr.value).into_owned(),
                                QName(b"Total") => total = String::from_utf8_lossy(&attr.value).into_owned(),
                                QName(b"Sello") => sello = String::from_utf8_lossy(&attr.value).into_owned(),
                                QName(b"NoCertificado") => no_certificado = String::from_utf8_lossy(&attr.value).into_owned(),
                                _ => (),
                            }
                        }
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
    }

    println!("NoCertificado: {}", no_certificado);
    println!("Folio: {}", folio);
    println!("Fecha: {}", fecha);
    println!("Moneda: {}", moneda);
    println!("Total: {}", total);
    println!("Sello: {}", sello);




    Ok(())
}
