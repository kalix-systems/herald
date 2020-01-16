use anyhow::Error;
use rustls::*;
use std::{fs, path::Path, sync::Arc};
use tokio_rustls::*;

pub struct TlsKeyPair {
    pub cert_der: Vec<u8>,
    pub priv_key: Vec<u8>,
}

impl TlsKeyPair {
    pub fn gen_new_self_signed<H: Into<Vec<String>>>(hosts: H) -> Result<Self, Error> {
        let cert = rcgen::generate_simple_self_signed(hosts)?;
        let cert_der = cert.serialize_der()?;
        let priv_key = cert.serialize_private_key_der();
        Ok(TlsKeyPair { cert_der, priv_key })
    }

    pub fn write_to_dir<P: AsRef<Path>>(
        &self,
        dir: &P,
    ) -> Result<(), std::io::Error> {
        let mut pb = dir.as_ref().to_path_buf();
        {
            pb.push("cert");
            fs::write(&pb, &self.cert_der)?;
            pb.pop();
        }
        {
            pb.push("priv");
            fs::write(&pb, &self.priv_key)?;
            pb.pop();
        }
        Ok(())
    }

    pub fn read_from_dir<P: AsRef<Path>>(dir: &P) -> Result<Self, std::io::Error> {
        let mut pb = dir.as_ref().to_path_buf();
        let cert_der;
        let priv_key;
        {
            pb.push("cert");
            cert_der = fs::read(&pb)?;
            pb.pop();
        }
        {
            pb.push("priv");
            priv_key = fs::read(&pb)?;
            pb.pop();
        }
        Ok(TlsKeyPair { cert_der, priv_key })
    }

    pub fn configure_server(&self) -> Result<TlsAcceptor, Error> {
        let mut config = rustls::ServerConfig::new(Arc::new(NoClientAuth));
        let pk = PrivateKey(self.priv_key.clone());
        config.set_single_cert(vec![Certificate(self.cert_der.clone())], pk)?;

        Ok(Arc::new(config).into())
    }
}
