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

    // Step 3: Ensure the root element is "Comprobante"
    if root.name() != "Comprobante" {
        println!("Expected root element 'Comprobante', found '{}'", root.name());
        return Ok(());
    }
    println!("------------------------------------------------------------------------");
    println!("Root Complemento: {}", root.name());
    // Step 4: Extract attributes from the "Comprobante" root element
    println!("Root element attributes:");
    let comprobante_attrs = [
        "SubTotal", "Version", "NoCertificado", "Total", "MetodoPago",
        "FormaPago", "Fecha", "TipoDeComprobante", "Certificado",
        "LugarExpedicion", "Moneda", "Sello", "Exportacion"
    ];
    for attr in &comprobante_attrs {
        if let Some(value) = root.attr(attr) {
            println!("{} = {}", attr, value);
        }
    }
    let binding = root.ns();
    let root_namespace = binding.as_str();

    // Step 5: Process the "Emisor" child
    println!("------------------------------------------------------------------------");
    println!("Processing Emisor");
    if let Some(emisor) = root.get_child("Emisor", root_namespace) {
        let emisor_attrs = ["RegimenFiscal", "Rfc", "Nombre"];
        println!("Emisor attributes:");
        for attr in &emisor_attrs {
            if let Some(value) = emisor.attr(attr) {
                println!("{} = {}", attr, value);
            }
        }
    }
    // Step 6: Process the "Receptor" child
    println!("------------------------------------------------------------------------");
    println!("Processing Receptor");
    if let Some(receptor) = root.get_child("Receptor", root_namespace) {
        let receptor_attrs = ["Nombre", "UsoCFDI", "DomicilioFiscalReceptor", "Rfc", "RegimenFiscalReceptor"];
        println!("Receptor attributes:");
        for attr in &receptor_attrs {
            if let Some(value) = receptor.attr(attr) {
                println!("{} = {}", attr, value);
            }
        }
    }

    // Step 7: Process multiple "Concepto" items in "Conceptos"
    println!("------------------------------------------------------------------------");
    println!("Processing Conceptos");
    if let Some(conceptos) = root.get_child("Conceptos", root_namespace) {
        for concepto in conceptos.children().filter(|c| c.name() == "Concepto") {
            let concepto_attrs = ["ObjetoImp", "ValorUnitario", "Importe", "ClaveProdServ", "Descripcion", "Cantidad", "ClaveUnidad"];
            println!("\tConcepto attributes:");
            for attr in &concepto_attrs {
                if let Some(value) = concepto.attr(attr) {
                    println!("\t\t{} = {}", attr, value);
                }
            }

            // Process "Traslado" under each "Concepto"
            if let Some(impuestos) = concepto.get_child("Impuestos", root_namespace) {
                if let Some(traslados) = impuestos.get_child("Traslados", root_namespace) {
                    for traslado in traslados.children().filter(|t| t.name() == "Traslado") {
                        let traslado_attrs = ["TasaOCuota", "Importe", "Base", "TipoFactor", "Impuesto"];
                        println!("\t\tTraslado attributes:");
                        for attr in &traslado_attrs {
                            if let Some(value) = traslado.attr(attr) {
                                println!("\t\t\t{} = {}", attr, value);
                            }
                        }
                    }
                }
            }
        }
    }

    // Step 8: Process "Impuestos" at the "Comprobante" level
    println!("------------------------------------------------------------------------");
    println!("Processing Impuestos");
    if let Some(impuestos) = root.get_child("Impuestos", root_namespace) {
        if let Some(total_impuestos) = impuestos.attr("TotalImpuestosTrasladados") {
            println!("TotalImpuestosTrasladados = {}", total_impuestos);
        }

        // Process "Traslado" under "Impuestos"
        if let Some(traslados) = impuestos.get_child("Traslados", root_namespace) {
            for traslado in traslados.children().filter(|t| t.name() == "Traslado") {
                let traslado_attrs = ["TasaOCuota", "Importe", "Base", "TipoFactor", "Impuesto"];
                println!("Impuestos Traslado attributes:");
                for attr in &traslado_attrs {
                    if let Some(value) = traslado.attr(attr) {
                        println!("{} = {}", attr, value);
                    }
                }
            }
        }
    }

    // Step 9: Process "Complemento" and "TimbreFiscalDigital"
    println!("------------------------------------------------------------------------");
    println!("Processing Complemento and TimbreFiscalDigital");
    if let Some(complemento) = root.get_child("Complemento", root_namespace) {
        if let Some(timbre) = complemento.get_child("TimbreFiscalDigital", "http://www.sat.gob.mx/TimbreFiscalDigital") {
            let timbre_attrs = ["Version", "NoCertificadoSAT", "FechaTimbrado", "RfcProvCertif", "SelloCFD", "UUID", "SelloSAT"];
            println!("TimbreFiscalDigital attributes:");
            for attr in &timbre_attrs {
                if let Some(value) = timbre.attr(attr) {
                    println!("{} = {}", attr, value);
                }
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    explore_root("test_data/T113_0.xml")?;
    Ok(())
}