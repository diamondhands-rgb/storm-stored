// Storage daemon (stored): microservice frontend for different storage backends
// used in LNP/BP nodes.
//
// Written in 2022 by
//     Dr. Maxim Orlovsky <orlovsky@lnp-bp.org>
//
// Copyright (C) 2022 by LNP/BP Standards Association, Switzerland.
//
// You should have received a copy of the MIT License along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

use internet2::addr::ServiceAddr;
use internet2::session::LocalSession;
use internet2::{
    CreateUnmarshaller, SendRecvMessage, TypedEnum, Unmarshall, Unmarshaller, ZmqSocketType,
};
use microservices::rpc::ServerError;
use microservices::ZMQ_CONTEXT;
use storm::{Chunk, ChunkId};

use crate::{FailureCode, Reply, Request, StoreReq};

pub struct Client {
    // TODO: Replace with RpcSession once its implementation is completed
    session_rpc: LocalSession,
    unmarshaller: Unmarshaller<Reply>,
}

impl Client {
    pub fn with(connect: &ServiceAddr) -> Result<Self, ServerError<FailureCode>> {
        debug!("Initializing runtime");

        trace!("Connecting to store daemon at {}", connect);
        let session_rpc =
            LocalSession::connect(ZmqSocketType::Req, connect, None, None, &ZMQ_CONTEXT)?;
        Ok(Self {
            session_rpc,
            unmarshaller: Reply::create_unmarshaller(),
        })
    }

    pub fn store(
        &mut self,
        db: String,
        data: impl AsRef<[u8]>,
    ) -> Result<ChunkId, ServerError<FailureCode>> {
        let chunk = Chunk::try_from(data.as_ref())?;
        let reply = self.request(Request::Store(StoreReq { db, chunk }))?;
        match reply {
            Reply::ChunkId(chunk_id) => Ok(chunk_id),
            Reply::Failure(failure) => Err(failure.into()),
            wrong => unreachable!("unexpected response {:?} from store service API", wrong),
        }
    }

    pub fn request(&mut self, request: Request) -> Result<Reply, ServerError<FailureCode>> {
        trace!("Sending request to the server: {:?}", request);
        let data = request.serialize();
        trace!("Raw request data ({} bytes): {:02X?}", data.len(), data);
        self.session_rpc.send_raw_message(&data)?;
        trace!("Awaiting reply");
        let raw = self.session_rpc.recv_raw_message()?;
        trace!("Got reply ({} bytes), parsing: {:02X?}", raw.len(), raw);
        let reply = self.unmarshaller.unmarshall(raw.as_slice())?;
        trace!("Reply: {:?}", reply);
        Ok((&*reply).clone())
    }
}
