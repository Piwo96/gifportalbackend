const anchor = require('@project-serum/anchor');
const { program } = require('@project-serum/anchor/dist/cjs/spl/token');
const { SystemProgram } = anchor.web3;

const main = async() => {
  console.log("🚀 Starting test...")

  // Create and set the provider. We set it before but we needed to update it, so that it can communicate with our frontend!
  const provider = anchor.Provider.env();
  // stores data from solana config get
  anchor.setProvider(provider);
  // compiles code and deploys it to the local validator
  const program = anchor.workspace.Gifportalbackend;

  // --> Alternative approach
  // Read the generated IDL.
  // const idl = JSON.parse(
  //   require("fs").readFileSync("./target/idl/gifportalbackend.json", "utf8")
  // );

  // // Address of the deployed program.
  // const programId = new anchor.web3.PublicKey("338BZQTraQYe4yLfDa6Cb4WMACeF3ccv3umBKTER7R96");

  // // Generate the program client from IDL.
  // const program = new anchor.Program(idl, programId);

  // // Execute the RPC.
  // const tx = await program.rpc.startStuffOff();

  // Create an account keypair for our program to use.
  
  const baseAccount = anchor.web3.Keypair.generate();

  // Call start_stuff_off, pass it the params it needs!
  let tx = await program.rpc.startStuffOff({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [baseAccount],
  });

  console.log("📝 Your transaction signature", tx);

  // Fetch data from the account.
  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 GIF Count', account.totalGifs.toString())

  // Call add_gif!
  await program.rpc.addGif(
    "https://media3.giphy.com/media/UdoZgS1p7nr7a/giphy.gif?cid=ecf05e47y3ccgvxeu9bun0aqhitfzeen92oo32kpmhe1si89&rid=giphy.gif&ct=g",
    {
      accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });

  await program.rpc.addGif(
    "https://media.giphy.com/media/HiNok3PiLOUbm/giphy.gif",
    {
      accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });
  
  // Get the account again to see what changed.
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 GIF Count', account.totalGifs.toString())

  // Upvote a gif
  await program.rpc.upvoteGif(
    "https://media3.giphy.com/media/UdoZgS1p7nr7a/giphy.gif?cid=ecf05e47y3ccgvxeu9bun0aqhitfzeen92oo32kpmhe1si89&rid=giphy.gif&ct=g",
    {
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
      }
    }
  );

  // Get account again to see what changed.
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);

  // Access gif_list on the account!
  console.log('👀 GIF List', account.gifList);

  // Delete gif_list on the account
  await program.rpc.removeGif(
    "https://media.giphy.com/media/HiNok3PiLOUbm/giphy.gif",
    {
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
      }
    }
  );

  await program.rpc.downvoteGif(
    "https://media3.giphy.com/media/UdoZgS1p7nr7a/giphy.gif?cid=ecf05e47y3ccgvxeu9bun0aqhitfzeen92oo32kpmhe1si89&rid=giphy.gif&ct=g",
    {
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
      }
    }
  );

  // Get account again to see what changed.
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 GIF List', account.gifList);
}

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (error) {
    console.error(error);
    process.exit(1);
  }
};

runMain();