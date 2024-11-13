mod models;

use minidom::Element;
use std::{fs::File, path};
use std::io::Read;
use std::error::Error;
use models::comprobante::{Comprobante, Emisor, Receptor, Concepto, Impuestos, Traslado, TimbreFiscalDigital};
use std::time::Instant;

fn explore_root(file_name: &str) -> Result<Comprobante, Box<dyn Error>> {
    // Step 1: Load the XML file content into a string
    let mut contents = String::new();
    File::open(file_name)?.read_to_string(&mut contents)?;

    // Step 2: Parse the content into an XML root element
    let root: Element = contents.parse()?;

    // Step 3: Ensure the root element is "Comprobante"
    if root.name() != "Comprobante" {
        //println!("Expected root element 'Comprobante', found '{}'", root.name());
        return Ok(Comprobante::default());
    }
    let mut comprobante = Comprobante::default();
    //println!("------------------------------------------------------------------------");
    //println!("Root Complemento: {}", root.name());
    // Step 4: Extract attributes from the "Comprobante" root element
    //println!("Root element attributes:");
    let comprobante_attrs = [
        "SubTotal", "Version", "NoCertificado", "Total", "MetodoPago",
        "FormaPago", "Fecha", "TipoDeComprobante", "Certificado",
        "LugarExpedicion", "Moneda", "Sello", "Exportacion"
    ];

    comprobante.version = root.attr("Version").unwrap().to_string();
    //comprobante.serie = root.attr("Serie").unwrap().to_string();
    //comprobante.folio = root.attr("Folio").unwrap().to_string();
    comprobante.fecha = root.attr("Fecha").unwrap().to_string();
    comprobante.sello = root.attr("Sello").unwrap().to_string();
    comprobante.forma_pago = root.attr("FormaPago").unwrap().to_string();
    comprobante.no_certificado = root.attr("NoCertificado").unwrap().to_string();
    comprobante.certificado = root.attr("Certificado").unwrap().to_string();
    comprobante.sub_total = root.attr("SubTotal").unwrap().to_string();
    comprobante.total = root.attr("Total").unwrap().to_string();
    comprobante.tipo_de_comprobante = root.attr("TipoDeComprobante").unwrap().to_string();
    comprobante.metodo_pago = root.attr("MetodoPago").unwrap().to_string();
    comprobante.lugar_expedicion = root.attr("LugarExpedicion").unwrap().to_string();

    for attr in &comprobante_attrs {
        if let Some(value) = root.attr(attr) {
            //println!("{} = {}", attr, value);
        }
    }
    let binding = root.ns();
    let root_namespace = binding.as_str();

    // Step 5: Process the "Emisor" child
    //println!("------------------------------------------------------------------------");
    //println!("Processing Emisor");

    let emisor = Emisor {
        rfc: root.get_child("Emisor", root_namespace).unwrap().attr("Rfc").unwrap().to_string(),
        nombre: root.get_child("Emisor", root_namespace).unwrap().attr("Nombre").unwrap().to_string(),
        regimen_fiscal: root.get_child("Emisor", root_namespace).unwrap().attr("RegimenFiscal").unwrap().to_string(),
    };

    comprobante.emisor = emisor;

    if let Some(emisor) = root.get_child("Emisor", root_namespace) {
        let emisor_attrs = ["RegimenFiscal", "Rfc", "Nombre"];
        //println!("Emisor attributes:");
        for attr in &emisor_attrs {
            if let Some(value) = emisor.attr(attr) {
                //println!("{} = {}", attr, value);
            }
        }
    }
    // Step 6: Process the "Receptor" child
    //println!("------------------------------------------------------------------------");
    //println!("Processing Receptor");

    let receptor = Receptor {
        rfc: root.get_child("Receptor", root_namespace).unwrap().attr("Rfc").unwrap().to_string(),
        nombre: root.get_child("Receptor", root_namespace).unwrap().attr("Nombre").unwrap().to_string(),
        uso_cfdi: root.get_child("Receptor", root_namespace).unwrap().attr("UsoCFDI").unwrap().to_string(),
        domicilio_fiscal: root.get_child("Receptor", root_namespace).unwrap().attr("DomicilioFiscalReceptor").unwrap().to_string(),
        regimen_fiscal: root.get_child("Receptor", root_namespace).unwrap().attr("RegimenFiscalReceptor").unwrap().to_string(),
    };

    comprobante.receptor = receptor;

    if let Some(receptor) = root.get_child("Receptor", root_namespace) {
        let receptor_attrs = ["Nombre", "UsoCFDI", "DomicilioFiscalReceptor", "Rfc", "RegimenFiscalReceptor"];
        //println!("Receptor attributes:");
        for attr in &receptor_attrs {
            if let Some(value) = receptor.attr(attr) {
                //println!("{} = {}", attr, value);
            }
        }
    }

    // Step 7: Process multiple "Concepto" items in "Conceptos"
    //println!("------------------------------------------------------------------------");
    //println!("Processing Conceptos");
    if let Some(conceptos) = root.get_child("Conceptos", root_namespace) {
        for concepto in conceptos.children().filter(|c| c.name() == "Concepto") {

            let mut concepto_obj = Concepto::default();
            concepto_obj.objeto_imp = concepto.attr("ObjetoImp").unwrap().to_string();
            concepto_obj.valor_unitario = concepto.attr("ValorUnitario").unwrap().to_string();
            concepto_obj.importe = concepto.attr("Importe").unwrap().to_string();
            concepto_obj.clave_prod_serv = concepto.attr("ClaveProdServ").unwrap().to_string();
            concepto_obj.descripcion = concepto.attr("Descripcion").unwrap().to_string();
            concepto_obj.cantidad = concepto.attr("Cantidad").unwrap().to_string();
            concepto_obj.clave_unidad = concepto.attr("ClaveUnidad").unwrap().to_string();

            let concepto_attrs = ["ObjetoImp", "ValorUnitario", "Importe", "ClaveProdServ", "Descripcion", "Cantidad", "ClaveUnidad"];
            //println!("\tConcepto attributes:");
            for attr in &concepto_attrs {
                if let Some(value) = concepto.attr(attr) {
                    //println!("\t\t{} = {}", attr, value);
                }
            }

            // Process "Traslado" under each "Concepto"
            if let Some(impuestos) = concepto.get_child("Impuestos", root_namespace) {
                if let Some(traslados) = impuestos.get_child("Traslados", root_namespace) {
                    
                    for traslado in traslados.children().filter(|t| t.name() == "Traslado") {

                        let mut traslado_obj = Traslado::default();

                        traslado_obj.tasa_o_cuota = traslado.attr("TasaOCuota").unwrap().to_string();
                        traslado_obj.importe = traslado.attr("Importe").unwrap().to_string();
                        traslado_obj.base = traslado.attr("Base").unwrap().to_string();
                        traslado_obj.tipo_factor = traslado.attr("TipoFactor").unwrap().to_string();
                        traslado_obj.impuesto = traslado.attr("Impuesto").unwrap().to_string();

                        concepto_obj.impuestos = traslado_obj;

                        let traslado_attrs = ["TasaOCuota", "Importe", "Base", "TipoFactor", "Impuesto"];
                        //println!("\t\tTraslado attributes:");
                        for attr in &traslado_attrs {
                            if let Some(value) = traslado.attr(attr) {
                                //println!("\t\t\t{} = {}", attr, value);
                            }
                        }
                    }
                }
            }
            
            comprobante.conceptos.push(concepto_obj);
        }
    }

    // Step 8: Process "Impuestos" at the "Comprobante" level
    //println!("------------------------------------------------------------------------");
    //println!("Processing Impuestos");
    if let Some(impuestos) = root.get_child("Impuestos", root_namespace) {
        let mut impuestos_obj = Impuestos::default();

        impuestos_obj.total_impuestos_trasladados = impuestos.attr("TotalImpuestosTrasladados").unwrap().to_string();

        if let Some(total_impuestos) = impuestos.attr("TotalImpuestosTrasladados") {
            //println!("TotalImpuestosTrasladados = {}", total_impuestos);
        }

        // Process "Traslado" under "Impuestos"
        if let Some(traslados) = impuestos.get_child("Traslados", root_namespace) {
            for traslado in traslados.children().filter(|t| t.name() == "Traslado") {

                let mut traslado_obj = Traslado::default();

                traslado_obj.tasa_o_cuota = traslado.attr("TasaOCuota").unwrap().to_string();
                traslado_obj.importe = traslado.attr("Importe").unwrap().to_string();
                traslado_obj.base = traslado.attr("Base").unwrap().to_string();
                traslado_obj.tipo_factor = traslado.attr("TipoFactor").unwrap().to_string();
                traslado_obj.impuesto = traslado.attr("Impuesto").unwrap().to_string();

                impuestos_obj.traslados.push(traslado_obj);

                let traslado_attrs = ["TasaOCuota", "Importe", "Base", "TipoFactor", "Impuesto"];
                //println!("Impuestos Traslado attributes:");
                for attr in &traslado_attrs {
                    if let Some(value) = traslado.attr(attr) {
                        //println!("{} = {}", attr, value);
                    }
                }
            }
        }

        comprobante.impuestos = impuestos_obj;
    }

    // Step 9: Process "Complemento" and "TimbreFiscalDigital"
    //println!("------------------------------------------------------------------------");
    //println!("Processing Complemento and TimbreFiscalDigital");
    if let Some(complemento) = root.get_child("Complemento", root_namespace) {
        if let Some(timbre) = complemento.get_child("TimbreFiscalDigital", "http://www.sat.gob.mx/TimbreFiscalDigital") {

            let mut timbre_obj = TimbreFiscalDigital::default();

            timbre_obj.version = timbre.attr("Version").unwrap().to_string();
            timbre_obj.no_certificado_sat = timbre.attr("NoCertificadoSAT").unwrap().to_string();
            timbre_obj.fecha_timbrado = timbre.attr("FechaTimbrado").unwrap().to_string();
            timbre_obj.rfc_prov_certif = timbre.attr("RfcProvCertif").unwrap().to_string();
            timbre_obj.sello_cfd = timbre.attr("SelloCFD").unwrap().to_string();
            timbre_obj.uuid = timbre.attr("UUID").unwrap().to_string();
            timbre_obj.sello_sat = timbre.attr("SelloSAT").unwrap().to_string();

            comprobante.timbre_fiscal_digital = timbre_obj;

            let timbre_attrs = ["Version", "NoCertificadoSAT", "FechaTimbrado", "RfcProvCertif", "SelloCFD", "UUID", "SelloSAT"];
            //println!("TimbreFiscalDigital attributes:");
            for attr in &timbre_attrs {
                if let Some(value) = timbre.attr(attr) {
                    //println!("{} = {}", attr, value);
                }
            }
        }
    }
    Ok(comprobante)
}

fn main() -> Result<(), Box<dyn Error>> {
    let comprobante =  explore_root("test_data/SLI010507984_00461_LZWE_14555_11132554.xml")?;

    //println!("Finished processing XML file - - - - - - - - - - - - - - - - - - - - - - - - - - - - ");

    //println!("{:#?}", comprobante);

    //now iterate over all the files in test_data folder that are xml files

    let paths = path::Path::new("test_data").read_dir()?;

    // filter only xml files

    let paths = paths.filter(|entry| {
        if let Ok(entry) = entry {
            if let Some(extension) = entry.path().extension() {
                if extension == "xml" {
                    return true;
                }
            }
        }
        false
    });


    for path in paths {
        println!("Executing for file: {:?}", path);
    
        // Try to unwrap the path, handle any errors that occur
        let path = match path {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Error accessing path: {:?}", e);
                continue; // Skip to the next iteration on error
            }
        };
    
        let file_name = path.path().display().to_string();
        let start = Instant::now();
    
        // Try to execute explore_root, handle errors if any
        match explore_root(&file_name) {
            Ok(comprobante) => {
                let duration = start.elapsed();
                println!("Execution time for explore_root: {:?}", duration);
                //println!("{:#?}", comprobante);
            },
            Err(e) => {
                eprintln!("Error processing file {}: {:?}", file_name, e);
            }
        }
    }

    Ok(())
}