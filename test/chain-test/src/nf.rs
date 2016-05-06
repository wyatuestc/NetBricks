use e2d2::headers::*;
use e2d2::packet_batch::*;

#[inline]
pub fn chain_nf<T: 'static + Batch>(parent: T) -> CompositionBatch {
    parent.parse::<MacHeader>()
          .parse::<IpHeader>()
          .transform(box move |h, _, _| {
              let ttl = h.ttl();
              h.set_ttl(ttl + 1);
          }).compose()
}

#[inline]
pub fn chain<T: 'static + Batch>(parent: T, len: u32) -> CompositionBatch {
    let mut chained = chain_nf(parent);
    for _ in 1..len {
        chained = chain_nf(chained);
    }
    chained
}
