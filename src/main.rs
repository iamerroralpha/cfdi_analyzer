mod models;

use minidom::Element;
use std::{fs::File, io::Read, path};
use std::error::Error;
use std::time::Instant;
use rayon::prelude::*; // Parallel processing with Rayon
use models::comprobante::{Comprobante, Concepto, Emisor, Impuestos, Receptor, TimbreFiscalDigital, Traslado};

fn parse_xml_file(file_name: &str) -> Result<Element, Box<dyn Error>> {
    let mut contents = String::new();
    File::open(file_name)?.read_to_string(&mut contents)?;

    let contents = contents.trim_start_matches('\u{feff}').trim_start();

    Ok(contents.parse()?)
}

fn extract_comprobante(root: &Element) -> Comprobante {
    let mut comprobante = Comprobante::default();
    let ns = root.ns();
    let root_namespace = ns.as_str();

    comprobante.version = root.attr("Version").unwrap().to_string();
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

    if let Some(emisor_elem) = root.get_child("Emisor", root_namespace) {
        comprobante.emisor = Emisor {
            rfc: emisor_elem.attr("Rfc").unwrap_or_default().to_string(),
            nombre: emisor_elem.attr("Nombre").unwrap_or_default().to_string(),
            regimen_fiscal: emisor_elem.attr("RegimenFiscal").unwrap_or_default().to_string(),
        };
    }

    if let Some(receptor_elem) = root.get_child("Receptor", root_namespace) {
        comprobante.receptor = Receptor {
            rfc: receptor_elem.attr("Rfc").unwrap_or_default().to_string(),
            nombre: receptor_elem.attr("Nombre").unwrap_or_default().to_string(),
            uso_cfdi: receptor_elem.attr("UsoCFDI").unwrap_or_default().to_string(),
            domicilio_fiscal: receptor_elem.attr("DomicilioFiscal").unwrap_or_default().to_string(),
            regimen_fiscal: receptor_elem.attr("RegimenFiscal").unwrap_or_default().to_string(),
        };
    }

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

                    }
                }
            }


            
            comprobante.conceptos.push(concepto_obj);
        }
    }

    if let Some(impuestos) = root.get_child("Impuestos", root_namespace) {
        let mut impuestos_obj = Impuestos::default();

        impuestos_obj.total_impuestos_trasladados = impuestos.attr("TotalImpuestosTrasladados").unwrap().to_string();

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

            }
        }
        comprobante.impuestos = impuestos_obj;
    }

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
        }
    }

    comprobante
}

fn process_file(file_name: &str) -> Result<Comprobante, Box<dyn Error>> {
    let root = parse_xml_file(file_name)?;
    let comprobante = extract_comprobante(&root);
    Ok(comprobante)
}

fn main() -> Result<(), Box<dyn Error>> {

    let paths: Vec<_> = path::Path::new("test_data")
        .read_dir()?
        .filter_map(Result::ok)
        .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "xml"))
        .collect();

        println!("Running sequential execution...");
        let sequential_start = Instant::now();
        for entry in &paths {
            let path = entry.path();
            let file_name = path.to_str().unwrap();
            let start = Instant::now();
            if let Err(e) = process_file(file_name) {
                eprintln!("Error processing file {}: {:?}", file_name, e);
            }
            println!("Processed {} in {:?}", file_name, start.elapsed());
        }
        println!(
            "Sequential execution completed in {:?}\n",
            sequential_start.elapsed()
        );
    
        println!("Running parallel execution...");
        let parallel_start = Instant::now();
        paths.par_iter().for_each(|entry| {
            let path = entry.path();
            let file_name = path.to_str().unwrap();
            let start = Instant::now();
            if let Err(e) = process_file(file_name) {
                eprintln!("Error processing file {}: {:?}", file_name, e);
            }
            println!("Processed {} in {:?}", file_name, start.elapsed());
        });
        println!(
            "Parallel execution completed in {:?}\n",
            parallel_start.elapsed()
        );
    

    Ok(())
}