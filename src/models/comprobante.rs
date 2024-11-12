
#[derive(Debug, Default)]
pub struct Comprobante {
    pub version: String,
    pub serie: String,
    pub folio: String,
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
    pub impuestos: Impuestos,
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