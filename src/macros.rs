macro_rules! wrap {
    ($val:ident, $res:ident) => (
        if $res == CUDA_SUCCESS {
            Ok($val)
        } else {
            Err($res)
        }
    )
}


