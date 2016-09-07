use uniform_types::*;
use std::os::raw::c_char;
use std::os::raw::c_int;
use std::marker::PhantomData;
use std::mem;
use util;

extern "C" {
    fn psyc_uniform_parse(uniform: *mut PsycUniform,
                          buffer: *const c_char,
                          length: usize)
                          -> c_int;
}

pub struct Uniform<'a> {
    uniform: PsycUniform,
    phantom: PhantomData<&'a Vec<u8>>
}

impl<'a> Uniform<'a> {
    pub fn from_bytes(buffer: &'a [u8]) -> Result<Self, UniformParseError> {
        let buffer_ptr = buffer.as_ptr() as *const c_char;
        let mut uniform: PsycUniform;
        unsafe {
            uniform = mem::zeroed();
            let uniform_ptr = &mut uniform as *mut PsycUniform;
            let parse_result = psyc_uniform_parse(uniform_ptr, buffer_ptr, buffer.len());
            if parse_result < PsycScheme::PSYC_SCHEME_PSYC as _ { 
                return Err(mem::transmute(parse_result))
            }
        }
        let result = Uniform {
            uniform: uniform,
            phantom: PhantomData
        };
        Ok(result)
    }

    //pub fn from_raw_uniform(uniform: PsycUniform) -> Self {
    //    UniformRef {
    //        uniform: uniform,
    //        phantom: PhantomData
    //    }
    //}

    pub fn is_valid(&self) -> bool {
        self.uniform.valid
    }

    pub fn scheme(&self) -> PsycScheme {
        self.uniform.uniform_type
    }

    pub fn name(&self) -> &'a str {
        unsafe {
            util::cstring_to_str(self.uniform.user.data, self.uniform.user.length)
        }
    }

    pub fn password(&self) -> &'a str {
        unsafe {
            util::cstring_to_str(self.uniform.pass.data, self.uniform.pass.length)
        }
    }

    pub fn host(&self) -> &'a str {
        unsafe {
            util::cstring_to_str(self.uniform.host.data, self.uniform.host.length)
        }
    }

    pub fn port(&self) -> Option<i32> {
        let slice: &'a str;
        unsafe {
            slice = util::cstring_to_str(self.uniform.port.data,
                                         self.uniform.port.length)
        }
        match slice {
            "" => None,
            s => Some(s.parse::<i32>().unwrap())
        }
    }

    pub fn transport(&self) -> Option<PsycTransport> {
        let slice: &'a str;
        unsafe {
            slice = util::cstring_to_str(self.uniform.transport.data,
                                         self.uniform.transport.length)
        }
        match slice {
            "" => None,
            tcp if tcp.as_bytes()[0] == PsycTransport::PSYC_TRANSPORT_TCP as u8 =>
                Some(PsycTransport::PSYC_TRANSPORT_TCP),
            udp if udp.as_bytes()[0] == PsycTransport::PSYC_TRANSPORT_UDP as u8 =>
                Some(PsycTransport::PSYC_TRANSPORT_UDP),
            tls if tls.as_bytes()[0] == PsycTransport::PSYC_TRANSPORT_TLS as u8 =>
                Some(PsycTransport::PSYC_TRANSPORT_TLS),
            gnunet if gnunet.as_bytes()[0] == PsycTransport::PSYC_TRANSPORT_GNUNET as u8 =>
                Some(PsycTransport::PSYC_TRANSPORT_GNUNET),
            _ => None
        }
    }

    pub fn resource(&self) -> &'a str {
        unsafe {
            util::cstring_to_str(self.uniform.resource.data,
                                 self.uniform.resource.length)
        }
    }

    pub fn query(&self) -> &'a str {
        unsafe {
            util::cstring_to_str(self.uniform.query.data, self.uniform.query.length)
        }
    }

    pub fn channel(&self) -> &'a str {
        unsafe {
            util::cstring_to_str(self.uniform.channel.data, self.uniform.channel.length)
        }
    }

    pub fn full(&self) -> &'a str {
        unsafe {
            util::cstring_to_str(self.uniform.full.data, self.uniform.full.length)
        }
    }

    pub fn body(&self) -> &'a str {
        unsafe {
            util::cstring_to_str(self.uniform.body.data, self.uniform.body.length)
        }
    }

    pub fn root(&self) -> &'a str {
        unsafe {
            util::cstring_to_str(self.uniform.root.data, self.uniform.root.length)
        }
    }

    pub fn entity(&self) -> &'a str {
        unsafe {
            util::cstring_to_str(self.uniform.entity.data, self.uniform.entity.length)
        }
    }

    pub fn path(&self) -> &'a str {
        unsafe {
            util::cstring_to_str(self.uniform.path.data, self.uniform.path.length)
        }
    }

    pub fn nick(&self) -> &'a str {
        unsafe {
            util::cstring_to_str(self.uniform.nick.data, self.uniform.nick.length)
        }
    }
}
