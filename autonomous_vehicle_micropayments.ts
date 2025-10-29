import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AutonomousVehiclePayments } from "../target/types/autonomous_vehicle_payments";
import { expect } from "chai";
import { PublicKey, SystemProgram, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { BN } from "@coral-xyz/anchor";

describe("autonomous_vehicle_payments", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.AutonomousVehiclePayments as Program<AutonomousVehiclePayments>;

  let authority: Keypair;
  let treasury: Keypair;
  let customer: Keypair;
  let vehicleOperator: Keypair;
  let configPDA: PublicKey;
  let vehiclePDA: PublicKey;
  let deliveryPDA: PublicKey;
  let escrowPDA: PublicKey;

  const vehicleId = "AV-001";
  const deliveryId = new BN(12345);
  const paymentAmount = new BN(1 * LAMPORTS_PER_SOL);
  const feeBps = 250;

  before(async () => {
    authority = Keypair.generate();
    treasury = Keypair.generate();
    customer = Keypair.generate();
    vehicleOperator = Keypair.generate();

    // Fund all accounts
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(authority.publicKey, 100 * LAMPORTS_PER_SOL)
    );
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(customer.publicKey, 100 * LAMPORTS_PER_SOL)
    );
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(vehicleOperator.publicKey, 100 * LAMPORTS_PER_SOL)
    );

    [configPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from("config"), authority.publicKey.toBuffer()],
      program.programId
    );

    [vehiclePDA] = PublicKey.findProgramAddressSync(
      [Buffer.from("vehicle"), Buffer.from(vehicleId)],
      program.programId
    );

    [deliveryPDA] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("delivery"),
        customer.publicKey.toBuffer(),
        deliveryId.toArrayLike(Buffer, "le", 8)
      ],
      program.programId
    );

    [escrowPDA] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("escrow"),
        customer.publicKey.toBuffer(),
        deliveryId.toArrayLike(Buffer, "le", 8)
      ],
      program.programId
    );
  });

  it("Initialize Config", async () => {
    await program.methods
      .initializeConfig(feeBps, treasury.publicKey)
      .accountsPartial({
        config: configPDA,
        authority: authority.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([authority])
      .rpc();

    const config = await program.account.config.fetch(configPDA);
    expect(config.isActive).to.be.true;
    expect(config.isPaused).to.be.false;
    expect(Number(config.feeBps)).to.equal(feeBps);
    expect(config.treasury.toString()).to.equal(treasury.publicKey.toString());
  });

  it("Register Vehicle", async () => {
    const location = "40.7128,-74.0060";

    await program.methods
      .registerVehicle(vehicleId, vehicleOperator.publicKey, location)
      .accountsPartial({
        vehicle: vehiclePDA,
        config: configPDA,
        authority: authority.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([authority])
      .rpc();

    const vehicle = await program.account.vehicle.fetch(vehiclePDA);
    expect(vehicle.vehicleId).to.equal(vehicleId);
    expect(vehicle.operator.toString()).to.equal(vehicleOperator.publicKey.toString());
    expect(vehicle.location).to.equal(location);
    expect(vehicle.isActive).to.be.true;
    expect(vehicle.isBusy).to.be.false;
    expect(Number(vehicle.totalDeliveries)).to.equal(0);
  });

  it("Create Delivery Order", async () => {
    const pickupLocation = "40.7128,-74.0060";
    const deliveryLocation = "40.7589,-73.9851";

    const customerBefore = await provider.connection.getBalance(customer.publicKey);

    await program.methods
      .createDeliveryOrder(
        deliveryId,
        paymentAmount,
        pickupLocation,
        deliveryLocation
      )
      .accountsPartial({
        delivery: deliveryPDA,
        escrow: escrowPDA,
        config: configPDA,
        customer: customer.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([customer])
      .rpc();

    const delivery = await program.account.delivery.fetch(deliveryPDA);
    expect(Number(delivery.deliveryId)).to.equal(Number(deliveryId));
    expect(delivery.customer.toString()).to.equal(customer.publicKey.toString());
    expect(Number(delivery.paymentAmount)).to.equal(Number(paymentAmount));
    expect(delivery.pickupLocation).to.equal(pickupLocation);
    expect(delivery.deliveryLocation).to.equal(deliveryLocation);
    expect(delivery.status).to.deep.equal({ pending: {} });
    expect(delivery.assignedVehicle).to.be.null;

    const customerAfter = await provider.connection.getBalance(customer.publicKey);
    const maxTxFee = 20000;
    expect(customerBefore - customerAfter).to.be.greaterThanOrEqual(Number(paymentAmount));
    expect(customerBefore - customerAfter).to.be.lessThanOrEqual(Number(paymentAmount) + maxTxFee);

    const escrowBalance = await provider.connection.getBalance(escrowPDA);
    expect(escrowBalance).to.equal(Number(paymentAmount));
  });

  it("Accept Delivery", async () => {
    await program.methods
      .acceptDelivery(deliveryId)
      .accountsPartial({
        delivery: deliveryPDA,
        vehicle: vehiclePDA,
        config: configPDA,
        operator: vehicleOperator.publicKey,
      })
      .signers([vehicleOperator])
      .rpc();

    const delivery = await program.account.delivery.fetch(deliveryPDA);
    expect(delivery.status).to.deep.equal({ inProgress: {} });
    expect(delivery.assignedVehicle.toString()).to.equal(vehiclePDA.toString());

    const vehicle = await program.account.vehicle.fetch(vehiclePDA);
    expect(vehicle.isBusy).to.be.true;
  });

  it("Complete Delivery", async () => {
    const vehicleOperatorBefore = await provider.connection.getBalance(vehicleOperator.publicKey);
    const treasuryBefore = await provider.connection.getBalance(treasury.publicKey);

    await program.methods
      .completeDelivery(deliveryId)
      .accountsPartial({
        delivery: deliveryPDA,
        escrow: escrowPDA,
        vehicle: vehiclePDA,
        vehicleOperator: vehicleOperator.publicKey,
        treasury: treasury.publicKey,
        config: configPDA,
        customer: customer.publicKey,
      })
      .signers([vehicleOperator])
      .rpc();

    const delivery = await program.account.delivery.fetch(deliveryPDA);
    expect(delivery.status).to.deep.equal({ completed: {} });

    const vehicle = await program.account.vehicle.fetch(vehiclePDA);
    expect(vehicle.isBusy).to.be.false;
    expect(Number(vehicle.totalDeliveries)).to.equal(1);

    const vehicleOperatorAfter = await provider.connection.getBalance(vehicleOperator.publicKey);
    const treasuryAfter = await provider.connection.getBalance(treasury.publicKey);

    const expectedFee = Number(paymentAmount) * feeBps / 10000;
    const expectedPayment = Number(paymentAmount) - expectedFee;

    expect(vehicleOperatorAfter - vehicleOperatorBefore).to.be.greaterThanOrEqual(expectedPayment);
    expect(treasuryAfter - treasuryBefore).to.be.greaterThanOrEqual(expectedFee);

    const escrowBalance = await provider.connection.getBalance(escrowPDA);
    expect(escrowBalance).to.be.lessThanOrEqual(10000);
  });

  it("Prevents duplicate vehicle registration", async () => {
    try {
      await program.methods
        .registerVehicle(vehicleId, vehicleOperator.publicKey, "new location")
        .accountsPartial({
          vehicle: vehiclePDA,
          config: configPDA,
          authority: authority.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([authority])
        .rpc();
      expect.fail("Should have failed");
    } catch (error) {
      expect(error.message).to.include("already in use");
    }
  });

  it("Prevents unauthorized delivery acceptance", async () => {
    const unauthorizedOperator = Keypair.generate();
    const newDeliveryId = new BN(54321);

    const [newDeliveryPDA] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("delivery"),
        customer.publicKey.toBuffer(),
        newDeliveryId.toArrayLike(Buffer, "le", 8)
      ],
      program.programId
    );

    const [newEscrowPDA] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("escrow"),
        customer.publicKey.toBuffer(),
        newDeliveryId.toArrayLike(Buffer, "le", 8)
      ],
      program.programId
    );

    await program.methods
      .createDeliveryOrder(
        newDeliveryId,
        paymentAmount,
        "pickup",
        "delivery"
      )
      .accountsPartial({
        delivery: newDeliveryPDA,
        escrow: newEscrowPDA,
        config: configPDA,
        customer: customer.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([customer])
      .rpc();

    try {
      await program.methods
        .acceptDelivery(newDeliveryId)
        .accountsPartial({
          delivery: newDeliveryPDA,
          vehicle: vehiclePDA,
          config: configPDA,
          operator: unauthorizedOperator.publicKey,
        })
        .signers([unauthorizedOperator])
        .rpc();
      expect.fail("Should have failed");
    } catch (error) {
      expect(error.message).to.include("unknown signer");
    }
  });
});
