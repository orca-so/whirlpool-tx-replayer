use solana_cli_output::display::println_transaction;
use solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta;


pub trait PrintableTransaction {
  /// Pretty print the transaction results, tagged with the given name for distinguishability.
  fn print_named(&self, name: &str);

  /// Pretty print the transaction results.
  fn print(&self) {
      self.print_named("");
  }

  /// Panic and print the transaction if it did not execute successfully
  fn assert_success(&self);
}

impl PrintableTransaction for EncodedConfirmedTransactionWithStatusMeta {
  fn print_named(&self, name: &str) {
      let tx = self.transaction.transaction.decode().unwrap();
      println!("EXECUTE {} (slot {})", name, self.slot);
      println_transaction(&tx, self.transaction.meta.as_ref(), "  ", None, None);
  }

  fn assert_success(&self) {
      match &self.transaction.meta {
          Some(meta) if meta.err.is_some() => {
              self.print();
              panic!("tx failed!")
          }
          _ => (),
      }
  }
}