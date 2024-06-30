use autocxx::prelude::*;

include_cpp! {
    #include "hpx/hpx_main.hpp"
    #include "hpx/iostream.hpp"
    safety!(unsafe)
    generate!("hpx::init")
    generate!("hpx::finalize")
    generate!("hpx::cout")
    // Add more generate! directives for other HPX functions/classes you want to use
}
