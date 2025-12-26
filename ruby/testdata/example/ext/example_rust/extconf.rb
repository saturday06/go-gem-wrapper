# frozen_string_literal: true

require "mkmf"
require "rb_sys/mkmf"

# Makes all symbols private by default to avoid unintended conflict
# with other gems. To explicitly export symbols you can use RUBY_FUNC_EXPORTED
# selectively, or entirely remove this flag.
append_cflags("-fvisibility=hidden")

create_rust_makefile("example/example_rust")
