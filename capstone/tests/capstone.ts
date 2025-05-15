// import * as anchor from "@coral-xyz/anchor";
// import { Program, AnchorProvider, web3 } from "@coral-xyz/anchor";
// import { PublicKey } from "@solana/web3.js";
// import { assert } from "chai";
// import { Capstone } from "../target/types/capstone";

// describe("capstone", () => {
//   // Configure the client to use the local cluster.
//   // anchor.setProvider(anchor.AnchorProvider.env());
//   const provider = AnchorProvider.env();
//   anchor.setProvider(provider);

//   const program = anchor.workspace.capstone as Program<Capstone>;
//   const doctor = provider.wallet;

//   // Helper function to generate a patient ID (random public key)
//   const generatePatientId = () => new web3.Keypair().publicKey.toBase58();

//   // Helper function to derive PDA
//   const getRecordPda = async (patientId: string, doctor: PublicKey) => {
//     return await PublicKey.findProgramAddress(
//       [Buffer.from(patientId), Buffer.from("record_entry")],
//       program.programId
//     );
//   };

//   it("Creates a health record successfully", async () => {
//     const patientId = generatePatientId();
//     const akaveCid =
//       "Qm1234567890abcdef1234567890abcdef1234567890abcdef12345678";

//     const [recordPda, bump] = await getRecordPda(patientId, doctor.publicKey);

//     await program.methods
//       .enterHealthRecord(patientId, akaveCid)
//       .accounts({
//         recordEntry: recordPda,
//         doctor: doctor.publicKey,
//         systemProgram: web3.SystemProgram.programId,
//       })
//       .rpc();

//     // Fetch the account and verify its contents
//     const recordAccount = await program.account.recordDetailsEntry.fetch(
//       recordPda
//     );

//     assert.equal(recordAccount.doctor.toBase58(), doctor.publicKey.toBase58());
//     assert.equal(recordAccount.patientId, patientId);
//     assert.equal(recordAccount.akaveCid, akaveCid);
//     assert.isNumber(recordAccount.timestamp);
//   });

//   it("Fails if patient_id is too long", async () => {
//     const patientId = "A".repeat(45); // Exceeds 44 chars
//     const akaveCid =
//       "Qm1234567890abcdef1234567890abcdef1234567890abcdef12345678";

//     const [recordPda] = await getRecordPda(patientId, doctor.publicKey);

//     try {
//       await program.methods
//         .enterHealthRecord(patientId, akaveCid)
//         .accounts({
//           recordEntry: recordPda,
//           doctor: doctor.publicKey,
//           systemProgram: web3.SystemProgram.programId,
//         })
//         .rpc();
//       assert.fail("Should have failed with PatientIdTooLong error");
//     } catch (err) {
//       assert.include(
//         err.toString(),
//         "Patient ID should be a valid Solana public key (max 44 chars)"
//       );
//     }
//   });

//   it("Fails if akave_cid is too long", async () => {
//     const patientId = generatePatientId();
//     const akaveCid = "Qm" + "A".repeat(63); // Exceeds 64 chars

//     const [recordPda] = await getRecordPda(patientId, doctor.publicKey);

//     try {
//       await program.methods
//         .enterHealthRecord(patientId, akaveCid)
//         .accounts({
//           recordEntry: recordPda,
//           doctor: doctor.publicKey,
//           systemProgram: web3.SystemProgram.programId,
//         })
//         .rpc();
//       assert.fail("Should have failed with AkaveCidTooLong error");
//     } catch (err) {
//       assert.include(err.toString(), "Akave CID too long (max 64 chars)");
//     }
//   });
// });



// DON'T KNOW HOW TO WRITE THE TESTS YET, BUT IT PASSES ON SOLANA PLAYGROUND