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
}

pub struct Emisor {
    pub rfc: String,
    pub nombre: String,
    pub regimen_fiscal: String,
}

pub struct Receptor {
    pub rfc: String,
    pub nombre: String,
    pub uso_cfdi: String,
    pub domicilio_fiscal: String,
    pub regimen_fiscal: String,
}