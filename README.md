# Autonomous Vehicle Payments - Solana IoT Orchestrator

A Solana-based IoT orchestrator enabling micropayments for autonomous vehicle delivery networks through smart contract automation.

## 🚗 Overview

This project implements a decentralized payment system for autonomous vehicle fleets that deliver goods in neighborhoods. The system uses Solana smart contracts to handle:

- **Escrow Payments**: Secure payment holding until delivery completion
- **Multi-Agent Coordination**: Vehicle registration and delivery assignment
- **Automatic Settlement**: Fee distribution and payment release upon delivery
- **IoT Integration**: Real-time coordination between vehicles and customers

## 🏗️ Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│     Customer    │    │    Vehicle      │    │   Authority     │
│                 │    │   Operator      │    │   (Platform)    │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         │                       │                       │
         ▼                       ▼                       ▼
┌──────────────────────────────────────────────────────────────┐
│                 Solana Blockchain                            │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐           │
│  │   Config    │ │   Vehicle   │ │  Delivery   │           │
│  │     PDA     │ │     PDA     │ │     PDA     │           │
│  └─────────────┘ └─────────────┘ └─────────────┘           │
│                                                            │
│  ┌─────────────┐                                          │
│  │   Escrow    │  ← Holds payment until delivery complete │
│  │     PDA     │                                          │
│  └─────────────┘                                          │
└──────────────────────────────────────────────────────────────┘
```

## 🚀 Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) 1.86+
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) 1.18+
- [Anchor](https://www.anchor-lang.com/docs/installation) 0.31.1
- [Node.js](https://nodejs.org/) 18+

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/your-org/autonomous-vehicle-payments.git
   cd autonomous-vehicle-payments
   ```

2. **Install dependencies**
   ```bash
   npm install
   ```

3. **Build the program**
   ```bash
   anchor build
   ```

4. **Run tests**
   ```bash
   anchor test
   ```

## 📋 Smart Contract Functions

### 1. Initialize Config
Sets up platform configuration with fees and treasury.

```typescript
await program.methods
  .initializeConfig(250, treasuryPublicKey) // 2.5% fee
  .rpc();
```

### 2. Register Vehicle
Registers an autonomous vehicle in the fleet.

```typescript
await program.methods
  .registerVehicle("AV-001", operatorPublicKey, "40.7128,-74.0060")
  .rpc();
```

### 3. Create Delivery Order
Customer creates a delivery order with escrowed payment.

```typescript
await program.methods
  .createDeliveryOrder(
    12345,                    // delivery ID
    1 * LAMPORTS_PER_SOL,     // payment amount
    "40.7128,-74.0060",       // pickup location
    "40.7589,-73.9851"        // delivery location
  )
  .rpc();
```

### 4. Accept Delivery
Vehicle operator accepts a pending delivery order.

```typescript
await program.methods
  .acceptDelivery(12345) // delivery ID
  .rpc();
```

### 5. Complete Delivery
Finalizes delivery and distributes payments automatically.

```typescript
await program.methods
  .completeDelivery(12345) // delivery ID
  .rpc();
```

## 💰 Payment Flow

1. **Order Creation**: Customer places order, payment escrowed on-chain
2. **Vehicle Assignment**: Available vehicle accepts delivery
3. **Delivery Execution**: Vehicle picks up and delivers goods
4. **Payment Settlement**: Smart contract automatically:
   - Pays vehicle operator (payment - platform fee)
   - Transfers platform fee to treasury
   - Updates vehicle delivery statistics

## 🧪 Testing

The test suite covers all core functionality:

```bash
# Run all tests
anchor test

# Run specific test
anchor test --grep "Complete Delivery"
```

### Test Coverage

- ✅ Config initialization with proper authorities
- ✅ Vehicle registration and validation
- ✅ Delivery order creation with escrow
- ✅ Delivery acceptance by authorized operators
- ✅ Payment distribution upon completion
- ✅ Error handling for unauthorized access
- ✅ Duplicate registration prevention

## 📊 Account Structure

### Config PDA
- **Seeds**: `["config", authority]`
- **Authority**: Platform administrator
- **Fields**: Fee rate, treasury, operational status

### Vehicle PDA
- **Seeds**: `["vehicle", vehicle_id]`
- **Authority**: Platform (registered by admin)
- **Fields**: Operator, location, availability, delivery count

### Delivery PDA
- **Seeds**: `["delivery", customer, delivery_id]`
- **Authority**: Customer
- **Fields**: Payment amount, locations, status, assigned vehicle

### Escrow PDA
- **Seeds**: `["escrow", customer, delivery_id]`
- **Authority**: Program
- **Purpose**: Holds customer payment until delivery completion

## 🔐 Security Features

- **Escrow Protection**: Payments held securely until delivery verified
- **Authority Validation**: Only authorized operators can accept deliveries
- **State Management**: Prevents double-spending and invalid state transitions
- **Math Safety**: Checked arithmetic prevents overflow attacks
- **Access Control**: Role-based permissions for different operations

## 🌐 IoT Integration

This smart contract serves as the payment layer for IoT devices. Integration points:

### Vehicle Integration
```javascript
// Vehicle IoT device accepts delivery
const tx = await program.methods
  .acceptDelivery(deliveryId)
  .accounts({
    operator: vehicleWallet.publicKey
  })
  .rpc();
```

### Customer App Integration
```javascript
// Customer mobile app creates order
const tx = await program.methods
  .createDeliveryOrder(orderId, amount, pickup, delivery)
  .accounts({
    customer: customerWallet.publicKey
  })
  .rpc();
```

## 📈 Usage Statistics

Track fleet performance:
- Total vehicles registered
- Deliveries completed per vehicle
- Average delivery fees collected
- Customer satisfaction metrics

## 🔧 Configuration

### Environment Variables
```bash
# Solana network
ANCHOR_PROVIDER_URL=https://api.devnet.solana.com
ANCHOR_WALLET=~/.config/solana/id.json

# Program settings
PLATFORM_FEE_BPS=250  # 2.5%
TREASURY_ADDRESS=your_treasury_pubkey
```

### Network Deployment

**Devnet:**
```bash
anchor deploy --provider.cluster devnet
```

**Mainnet:**
```bash
anchor deploy --provider.cluster mainnet-beta
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Guidelines

- Follow Anchor best practices
- Add tests for new features
- Update documentation
- Use meaningful commit messages
- Ensure all tests pass

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙋‍♂️ Support

- **Documentation**: https://docs.plena.finance
- **Issues**: Open a GitHub issue
- **Community**: Join our Discord server
- **Security**: Report vulnerabilities to security@plena.finance

## 🎯 Roadmap

- [ ] **V1.1**: Multi-token payment support
- [ ] **V1.2**: Delivery time predictions
- [ ] **V1.3**: Customer rating system
- [ ] **V1.4**: Fleet optimization algorithms
- [ ] **V2.0**: Cross-chain payment support

## 📄 Program ID

```
Devnet: 11111111111111111111111111111112
Mainnet: TBD
```

---

Built with ❤️ using Solana, Anchor, and TypeScript by [Plena Finance](https://plena.finance)
