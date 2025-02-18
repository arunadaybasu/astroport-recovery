# **Step-by-Step Plan for Recovering and Enhancing Astroport Classic on Terra Classic**

This is a **detailed breakdown** of our **plan**, covering **recovery, governance updates, liquidity mining restoration, fee incentives, and ASTROC tokenomics improvements**.

---

## **📌 Step 1: Recover Admin Control Over Astroport Classic**
### **Goal:** Regain admin access to control the liquidity pools, fee structures, and governance of Astroport Classic.

### **🔹 1.1 Identify the Current Admin Wallet**
- Check which **wallet currently controls the contracts** (if it's lost or inactive).
- Use **contract queries** to check the **admin role** in key contracts:
  ```bash
  terrad query wasm contract-state smart <contract_address> '{"get_admin":{}}'
  ```

### **🔹 1.2 Deploy the Governance Admin Change Contracts**
- Which contains governance contracts allowing **admin wallet changes** via **governance proposals**.
- This will allow **ASTROC holders** to vote on **changing the admin wallet**.

### **🔹 1.3 Propose an Admin Wallet Change**
- Submit a **governance proposal** to **change the admin wallet**:
  ```json
  {
    "propose": {
      "title": "Transfer Admin Rights",
      "description": "Change admin wallet to a new controlled address.",
      "proposal_type": "AdminChange",
      "new_admin": "terra1newadminaddress...",
      "duration": "7d"
    }
  }
  ```
- If the proposal **passes**, the **new wallet** will control all **Astroport Classic** contracts.

### **🔹 1.4 Verify Admin Control**
- Check if the new wallet **has control** over the contracts:
  ```bash
  terrad query wasm contract-state smart <contract_address> '{"get_admin":{}}'
  ```

✅ **End Result:** We now have **full admin control** over Astroport Classic.

---

## **📌 Step 2: Restore Liquidity Mining Rewards**
### **Goal:** Restart liquidity incentives using **ASTROC rewards** for liquidity providers.

### **🔹 2.1 Modify the Liquidity Mining Contracts**
- The **Generator contract** handles **liquidity mining rewards**.
- Verify if **ASTROC rewards** are being distributed.
  ```bash
  terrad query wasm contract-state smart <generator_contract_address> '{"get_rewards":{}}'
  ```

### **🔹 2.2 Set Up a New Liquidity Reward Program**
- **Distribute ASTROC rewards** to LPs (Liquidity Providers) in specific pools (e.g., **ASTROC-LUNC**).
- Adjust reward emission rates:
  ```json
  {
    "update_rewards": {
      "pool": "terra1astrocluncpool...",
      "rewards_per_block": "1000"
    }
  }
  ```

### **🔹 2.3 Relaunch Incentivized Liquidity Pools**
- **Restart LP incentives** so users **earn ASTROC** by adding liquidity.

✅ **End Result:** Liquidity providers **start earning ASTROC**, increasing **DEX liquidity**.

---

## **📌 Step 3: Recover and Reallocate Fee Revenues**
### **Goal:** Control trading fees and redirect them to **LP rewards, burns, and treasury funding**.

### **🔹 3.1 Modify the Fee Collection Mechanism**
- The **Maker contract** handles **fee collection and distribution**.
- Check where **fees are currently being sent**:
  ```bash
  terrad query wasm contract-state smart <maker_contract_address> '{"get_fee_distribution":{}}'
  ```

### **🔹 3.2 Adjust Fee Distribution for Sustainability**
- Set **new fee rules**:  
  - **30% to LP rewards**
  - **30% to Treasury**
  - **40% to ASTROC burns**
  ```json
  {
    "update_fee_distribution": {
      "lp_rewards": "30",
      "treasury": "30",
      "burn": "40"
    }
  }
  ```

✅ **End Result:** Fees now **fund liquidity incentives, treasury, and burns**.

---

## **📌 Step 4: Implement ASTROC Burn Mechanisms**
### **Goal:** Reduce ASTROC supply and increase scarcity.

### **🔹 4.1 Enable Auto-Burns in Swap Contracts**
- Use swap smart contracts so **every trade burns a portion of ASTROC**.
- Set a **burn fee** of **0.15% for ASTROC swaps**.
  ```json
  {
    "update_swap_fees": {
      "asset": "ASTROC",
      "burn_fee": "0.15"
    }
  }
  ```

### **🔹 4.2 Implement Fee-Based Auto-Burns**
- Modify the **Maker contract** to **burn part of the trading fees**:
  ```json
  {
    "update_fee_distribution": {
      "burn": "40"
    }
  }
  ```
✅ **End Result:** ASTROC is **burned continuously**, reducing supply.

---

## **📌 Step 5: Implement Developer & Treasury Funding**
### **Goal:** Fund development, maintenance, and governance.

### **🔹 5.1 Enable Developer Fees in Swap Contracts**
- Use **astroport-swap-fees.zip** to collect **0.1% developer fees** on LUNC and **0.15% on ASTROC swaps**.
  ```json
  {
    "update_swap_fees": {
      "asset": "LUNC",
      "dev_fee": "0.1"
    }
  }
  ```

### **🔹 5.2 Direct Treasury Funding from Fees**
- Allocate **30% of collected fees** to the **treasury**.

✅ **End Result:** Sustainable **funding for future development**.

---

## **📌 Final Summary**
| **Step** | **Action** | **Outcome** |
|----------|------------|-------------|
| **1** | Recover Admin Wallet | Full control over Astroport Classic |
| **2** | Restore Liquidity Mining | LPs start earning ASTROC |
| **3** | Reallocate Fees | Fees are used for rewards, treasury, and burns |
| **4** | Enable ASTROC Burns | Supply is reduced, increasing value |
| **5** | Implement Dev & Treasury Funding | Sustainable funding for future upgrades |

---
