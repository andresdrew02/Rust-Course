mod exercises {
    pub mod matches;
    pub mod copy;
    pub mod advanced_enum;
    pub mod options;
    pub mod demo;
    pub mod a17;
}

use exercises::matches;
use exercises::copy;
use exercises::advanced_enum;
use exercises::options;
use exercises::demo;
use exercises::a17;


fn main ()
{
    matches::exec();
    copy::exec();
    advanced_enum::exec();
    options::exec();
    demo::exec();
    a17::exec();
    // https://youtu.be/lzKeecy4OmQ?si=l-rbKzV7Hvu-Pbp-&t=18539
}