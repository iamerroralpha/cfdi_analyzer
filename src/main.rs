use std::fs::File;
use std::io::Read;
use quick_xml::events::Event;
use quick_xml::name::QName;
use quick_xml::reader::Reader;

#[derive(Default, Debug)]
struct CfdiComprobante {
    folio: String,
    fecha: String,
    moneda: String,
    total: String,
    sello: String,
    no_certificado: String,
    sub_total: String,
    version: String,
    metodo_pago: String,
    forma_pago: String,
    tipo_de_comprobante: String,
    certificado: String,
    lugar_expedicion: String,
    exportacion: String,
}

impl CfdiComprobante {
    fn display(&self) {
        println!("Folio: {}", self.folio);
        println!("Fecha: {}", self.fecha);
        println!("Moneda: {}", self.moneda);
        println!("Total: {}", self.total);
        println!("Sello: {}", self.sello);
        println!("NoCertificado: {}", self.no_certificado);
        println!("SubTotal: {}", self.sub_total);
        println!("Version: {}", self.version);
        println!("MetodoPago: {}", self.metodo_pago);
        println!("FormaPago: {}", self.forma_pago);
        println!("TipoDeComprobante: {}", self.tipo_de_comprobante);
        println!("Certificado: {}", self.certificado);
        println!("LugarExpedicion: {}", self.lugar_expedicion);
        println!("Exportacion: {}", self.exportacion);
    }
}

struct CfdiParser {
    file_name: String,
}

impl CfdiParser {
    fn new(file_name: &str) -> Self {
        CfdiParser { file_name: file_name.to_string() }
    }

    fn parse(&self) -> Result<CfdiComprobante, Box<dyn std::error::Error>> {
        let mut contents = String::new();
        File::open(&self.file_name)?.read_to_string(&mut contents)?;
        
        let mut reader = Reader::from_str(&contents);
        let mut comprobante = CfdiComprobante::default();

        loop {
            match reader.read_event() {
                Ok(Event::Start(ref e)) if e.name() == QName(b"cfdi:Comprobante") => {
                    for attr in e.attributes().flatten() {
                        let value = String::from_utf8_lossy(&attr.value).into_owned();
                        match attr.key {
                            QName(b"Folio") => comprobante.folio = value,
                            QName(b"Fecha") => comprobante.fecha = value,
                            QName(b"Moneda") => comprobante.moneda = value,
                            QName(b"Total") => comprobante.total = value,
                            QName(b"Sello") => comprobante.sello = value,
                            QName(b"NoCertificado") => comprobante.no_certificado = value,
                            QName(b"SubTotal") => comprobante.sub_total = value,
                            QName(b"Version") => comprobante.version = value,
                            QName(b"MetodoPago") => comprobante.metodo_pago = value,
                            QName(b"FormaPago") => comprobante.forma_pago = value,
                            QName(b"TipoDeComprobante") => comprobante.tipo_de_comprobante = value,
                            QName(b"Certificado") => comprobante.certificado = value,
                            QName(b"LugarExpedicion") => comprobante.lugar_expedicion = value,
                            QName(b"Exportacion") => comprobante.exportacion = value,
                            _ => (),
                        }
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
        }

        Ok(comprobante)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = CfdiParser::new("test_data/T113_0.xml");
    let comprobante = parser.parse()?;
    comprobante.display();
    Ok(())
}