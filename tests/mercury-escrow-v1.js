const assert = require("assert");
const anchor = require("@project-serum/anchor");
const { SystemProgram } = anchor.web3;

describe("mercury escrow", () => {
  // Use a local provider.
  const provider = anchor.AnchorProvider.local();

  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  // The Account to create.
  const myAccount = anchor.web3.Keypair.generate();
  const user = anchor.web3.Keypair.generate();

  const connection = anchor.getProvider().connection;

  before(async () => {
    let res = await connection.requestAirdrop(user.publicKey, 100 * anchor.web3.LAMPORTS_PER_SOL);

    let latestBlockHash = await connection.getLatestBlockhash()

    await connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: res,
    });
  });



  it("Creates and initializes an account in a single atomic transaction (simplified)", async () => {
    // #region code-simplified
    // The program to execute.
    const program = anchor.workspace.MercuryEscrowV1;



    // Create the new account and initialize it with the program.
    // #region code-simplified
    await program.methods.initialize(new anchor.BN(1234))
      .accounts({
        myAccount: myAccount.publicKey,
        user: user.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([myAccount, user])
      .rpc();
    // #endregion code-simplified

    // Fetch the newly created account from the cluster.
    const account = await program.account.myAccount.fetch(myAccount.publicKey);

    // Check it's state was initialized.
    assert.ok(account.data.eq(new anchor.BN(1234)));

    // Store the account for the next test.
    _myAccount = myAccount;
  });

  // it("Updates a previously created account", async () => {
  //   const myAccount = _myAccount;

  //   // #region update-test

  //   // The program to execute.
  //   const program = anchor.workspace.MercuryEscrowV1;

  //   // Invoke the update rpc.
  //   await program.methods
  //     .update(new anchor.BN(4321))
  //     .accounts({
  //       myAccount: myAccount.publicKey,
  //     })
  //     .rpc();

  //   // Fetch the newly updated account.
  //   const account = await program.account.myAccount.fetch(myAccount.publicKey);

  //   // Check it's state was mutated.
  //   assert.ok(account.data.eq(new anchor.BN(4321)));

  //   // #endregion update-test
  // });
});