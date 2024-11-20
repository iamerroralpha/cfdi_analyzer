use core::fmt;

#[derive(Debug, Default)]
pub struct Comprobante {
    pub version: String,
    pub fecha: String,
    pub sello: String,
    pub forma_pago: String,
    pub no_certificado: String,
    pub certificado: String,
    pub sub_total: String,
    pub total: String,
    pub tipo_de_comprobante: String,
    pub metodo_pago: String,
    pub lugar_expedicion: String,
    pub emisor: Emisor,
    pub receptor: Receptor,
    pub conceptos: Vec<Concepto>,
    pub impuestos: Impuestos,
    pub timbre_fiscal_digital: TimbreFiscalDigital,
}

impl Comprobante {
    //create a function that returns HEADERS_1 concatenated to HEADERS_2
    pub fn get_headers() -> String {
        format!("{},{}", HEADERS_1, HEADERS_2)
    }
}

pub const HEADERS_1: &str = "Archivo,Version,Fecha,FormaPago,SubTotal,Total,TipoDeComprobante,MetodoPago,LugarExpedicion,EmisorRFC,EmisorNombre,EmisorRegimenFiscal,ReceptorRFC,ReceptorNombre,ReceptorUsoCFDI,ReceptorRegimenFiscal,TotalImpuestosTrasladados,TimbreUUID,TimbreFechaTimbrado,TimbreRFCProvCertif,TimbreNoCertificadoSAT";

impl fmt::Display for Comprobante {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            ",{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
            self.version,
            self.fecha,
            self.forma_pago,
            self.sub_total,
            self.total,
            self.tipo_de_comprobante,
            self.metodo_pago,
            self.lugar_expedicion,
            self.emisor.rfc,
            self.emisor.nombre,
            self.emisor.regimen_fiscal,
            self.receptor.rfc,
            self.receptor.nombre,
            self.receptor.uso_cfdi,
            self.receptor.regimen_fiscal,
            self.impuestos.total_impuestos_trasladados,
            self.timbre_fiscal_digital.uuid,
            self.timbre_fiscal_digital.fecha_timbrado,
            self.timbre_fiscal_digital.rfc_prov_certif,
            self.timbre_fiscal_digital.no_certificado_sat,
        )
    }
}

#[derive(Debug, Default)]
pub struct Emisor {
    pub rfc: String,
    pub nombre: String,
    pub regimen_fiscal: String,
}

#[derive(Debug, Default)]
pub struct Receptor {
    pub rfc: String,
    pub nombre: String,
    pub uso_cfdi: String,
    pub domicilio_fiscal: String,
    pub regimen_fiscal: String,
}

#[derive(Debug, Default)]
pub struct Concepto {
    pub objeto_imp: String,
    pub valor_unitario: String,
    pub importe: String,
    pub clave_prod_serv: String,
    pub descripcion: String,
    pub cantidad: String,
    pub clave_unidad: String,
    pub impuestos: Traslado,
}

pub const HEADERS_2: &str = "ClaveProdServ,Descripcion,Cantidad,ValorUnitario,TasaOCuota,Importe,Base,TipoFactor,Impuesto";

impl fmt::Display for Concepto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            ",,,,,,,,,,,,,,,,,,,,,{},{},{},{}",
            self.clave_prod_serv,
            self.descripcion,
            self.cantidad,
            self.valor_unitario
        )
    }
}

#[derive(Debug, Default)]
pub struct Impuestos {
    pub total_impuestos_trasladados: String,
    pub traslados: Vec<Traslado>,
}

#[derive(Debug, Default)]
pub struct Traslado {
    pub tasa_o_cuota: String,
    pub importe: String,
    pub base: String,
    pub tipo_factor: String,
    pub impuesto: String,
}

impl fmt::Display for Traslado {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            ",{},{},{},{},{}",
            self.tasa_o_cuota, self.importe, self.base, self.tipo_factor, self.impuesto
        )
    }
}

#[derive(Debug, Default)]
pub struct TimbreFiscalDigital {
    pub version: String,
    pub uuid: String,
    pub fecha_timbrado: String,
    pub rfc_prov_certif: String,
    pub sello_cfd: String,
    pub no_certificado_sat: String,
    pub sello_sat: String,
}