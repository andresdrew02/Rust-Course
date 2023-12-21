mod exercises {
    pub mod matches;
    pub mod copy;
    pub mod advanced_enum;
    pub mod options;
}

use exercises::matches;
use exercises::copy;
use exercises::advanced_enum;
use exercises::options;


fn main ()
{
    matches::exec();
    copy::exec();
    advanced_enum::exec();
    options::exec();
    // https://youtu.be/lzKeecy4OmQ?si=bl0BUXe6Qwv1CMNB&t=17971
}