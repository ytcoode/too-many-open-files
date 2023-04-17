pub const AFTER_HELP: &str = color_print::cstr!(
    "<bold><underline>Examples:</underline></bold>

  Start the server:

    <bold>./too-many-open-files --server --server-bind-to=127.0.0.1:9999</bold>

  Start the client:

    <bold>./too-many-open-files --client --client-connect-to=127.0.0.1:9999\n</bold>
"
);
