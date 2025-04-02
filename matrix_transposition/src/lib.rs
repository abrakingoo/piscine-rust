struct Matrix((i32, i32), (i32, i32));
pub fn transpose(m: Matrix) -> Matrix {
    let Matrix((a,b), (c,d)) = m;
    Matrix((a,c),(b,d))
}