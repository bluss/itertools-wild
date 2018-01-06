
Itertools-wild
==============

Extra wild iterator adaptors, functions and wild macros.

Please read the `API documentation here`__

__ https://docs.rs/itertools-wild/

|build_status|_ |crates|_

.. |build_status| image:: https://travis-ci.org/bluss/itertools-wild.svg?branch=master
.. _build_status: https://travis-ci.org/bluss/itertools-wild

.. |crates| image:: http://meritbadge.herokuapp.com/itertools-wild
.. _crates: https://crates.io/crates/itertools-wild

How to use with cargo:

.. code:: toml

    [dependencies]
    itertools-wild = "*"

How to use in your crate:

.. code:: rust

    #[macro_use] extern crate itertools_wild;

    use itertools_wild::ItertoolsWild;

How to contribute:

- Fix a bug or implement a new thing
- Include tests for your new feature, preferably a quickcheck test
- Make a Pull Request


Recent Changes
--------------

- 0.1.1

  - Update authors: .feedback() is originally by @jameysharp

- 0.1.0

  - Initial release


License
-------

Dual-licensed to be compatible with the Rust project.

Licensed under the Apache License, Version 2.0
http://www.apache.org/licenses/LICENSE-2.0 or the MIT license
http://opensource.org/licenses/MIT, at your
option. This file may not be copied, modified, or distributed
except according to those terms.
