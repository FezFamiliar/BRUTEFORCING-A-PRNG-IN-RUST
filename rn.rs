extern crate rayon;
use rayon::prelude::*;
fn main(){

	let _maxseed = 0..4295032859u64;
	//let _m = 0..4u64;
	fn setup(mut _seed: u64) -> u64{
	let mut state = 0;
	for _i in 0..16 {
			 let _cur = _seed & 3;
			 _seed >>= 2;
			 state = (state << 4) | ((state & 3) ^ _cur);
			 state |= _cur << 2;
			// println!("{}", state);
		}
		return state;
	}

	fn next(_bits: u64,mut _state:u64) -> (u64,u64){
	let mut _ret = 0;
	for _i in 0.._bits {
		_ret <<= 1;
		_ret |= _state & 1;
		_state = (_state << 1) ^ (_state >> 61);
		_state &= std::u64::MAX;
		_state ^= std::u64::MAX;

		for j in (0..64).step_by(4){
			let mut _cur = (_state >> j) & 0xF;
			_cur = (_cur >> 3) | ((_cur >> 2) & 2) | ((_cur << 3) & 8) | ((_cur << 2) & 4);
			_state ^= _cur << j;
		}
	}
	return (_ret,_state)
   }
   

	fn try_seed(_i:u64){
   		
   		let state = setup(_i);
   		let (nxt1,state) = next(26,state);

		if nxt1 == 50870229{

			let (nxt2,state) = next(26,state);
			if nxt2 == 47380273{
				
				let (nxt3,state) = next(26,state);
				println!("Found! seed: {} nxt1: {} nxt2: {} nxt3 {}", _i,nxt1,nxt2,nxt3);
			}
			
		}

   	}

   	// println!("{}", next(26,setup(2149723668)));


   	_maxseed.into_par_iter().for_each(| _maxseed | try_seed(_maxseed));  

}