///args=2 args=Iterator 必须后位写数量
///args=N args=? 无所谓
#[macro_export]
macro_rules! epsilon_iterator {
    ($($a:expr),+ $(,)?)=>{
		$crate::iterator::Epsilon([$($a),*])
    };
    ($($($e:expr),*),+ $(,)?)=>{
        $crate::iterator::Epsilon([$($($e),*),*])
    };
//	($a:expr,$b:expr)=>{
//		$crate::iterator::Epsilon::<_, $b>::from_iter($a)
//	};
}
///# alpha
#[macro_export]
macro_rules! alpha {
    ($e:block) => {
        Box::pin(async move { $e })
    };
    ($($e:expr),*) => {
        Box::pin(async move { $($e)* });
    };
}
///# Beta
#[macro_export]
macro_rules! beta {
    ($($a:ident,$b:ty),*,$i:block)=>{
        Box::new(move |$($a: $b),*| {
            Box::pin(async move { $i
            })
        })
    };
    ($($a:ident,$b:ty),*,$i:block,$($e:expr),*)=>{
        Box::new(move |$($a: $b),*| {
            $($e)* Box::pin(async move { $i
            })
        })
    };
}
///# Gamma
#[macro_export]
macro_rules! gamma{
    ($($a:ident,$b:ty),*,$e:block) => {
        Box::new(move |$($a: $b),*| $e)
    };
    ($($a:ident,$b:ty),*,$($e:expr),*) => {
        Box::new(move |$($a: $b),*| {$($e)*})
    };
}
