/*!
   The legacy versions of genesis CLI commands for Cosmos chains.

   In the legacy versions, all genesis subcommands are direct subcommands
   of the main chain command. In the new version of Cosmos SDK CLI, the
   genesis subcommands are grouped under a `genesis` subcommand.
*/

pub mod add_genesis_account;
pub mod add_genesis_validator;
pub mod collect_gentxs;
