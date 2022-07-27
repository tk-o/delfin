# Delfin

The Delfin software is aiming to solve for processing and tracking for financial transactions of any kind. It makes the reporting for tax purposes easy and accurate, especially in Australian tax jurisdiction.

## Problem

As of now, I have to pull various data reports from trading and financial platforms that I use and have them shared with a tax accounting specialist so they could prepare a tax report for me.

There are various classes of taxable events, but it's not hard to determine which class should be applied to which event. The problem is the vast amount of data that needs to be processed.

```mermaid
flowchart TD
  Actor_FMI(((Market's investor)))
  Actor_TA(((Tax accountant)))
  Actor_BO(((Business owner)))

  subgraph Sys_Ext_TP [Trading Platform]
    Sys_Ext_TP_Op[Tracks trading operations\ni.e. assets purchases and disposals, trading fees]
  end

  subgraph Sys_Ext_FP [Financial Platform]
    Sys_Ext_FP_Op[Tracks financial operations\ni.e. inflows and outflows]
  end

  Sys_Ext_TAP[[Tax Accounting Platform]]

  Actor_FMI -->|writes operations| Sys_Ext_TP

  Actor_BO -->|writes opertions| Sys_Ext_FP

  Sys_Ext_TP & Sys_Ext_FP -->|reads operations|Actor_TA

  subgraph Sys_Ext_TAP [Tax Accounting Platform]
    Sys_Ext_TAP_Report[Creates financial reports\ni.e. tax reports, closing of the month, etc.]
  end

  Actor_TA -->|aggregates operations| Sys_Ext_TAP
```

## Looking for a solution

That's what I'm trying to address: processing data about financial operations so I could save my time and money (by saving time required for tax accounting).

I'd like to cover at least the areas of:

- sourcing financial data
  - this is where I usually need to grant access to some spreadsheets or other documents presenting all relevant transactions
- storing financial operations and transactions (where a single transaction includes at least one operation)
  - right now, I do that using spreadsheets in the cloud which is a manual and error-prone process
- accounting for inflows and outflows
  - I have to go through all transactions and ensure they have a correct label attached so my accountant knows which tax rulling to apply
- tax reporting for Australian tax law
  - this is where the tax accountant specialst use their tax law knowledge (and usually also their spreadsheet know-how)
  - can be a very costly process and error-prone process — the more transactions the more expensive the process gets, and the longer it is the more errors may show up in the calculations

## Minimal Viable Product

The MVP scope expressed in the [C4 model](https://c4model.com).

### System Context diagram

```mermaid
flowchart TD
  Actor_FMI(((Market's investor)))
  Actor_TA(((Tax accountant)))
  Actor_BO(((Business owner)))
  Actor_SysOp(((System operator)))

  subgraph Sys_Ext_TP ["(External) Trading Platform"]
    Sys_Ext_TP_Op[Tracks trading operations\ni.e. assets purchases and disposals, trading fees]
  end

  subgraph Sys_Ext_FP ["(External) Financial Platform"]
    Sys_Ext_FP_Op[Tracks financial operations\ni.e. inflows and outflows]
  end

  Actor_FMI -->|writes operations| Sys_Ext_TP

  Actor_BO -->|writes opertions| Sys_Ext_FP

  subgraph Sys_Int_FOR ["(Internal) Finance on Rails Platform"]
    subgraph Modules
      Sys_Int_FOR_Imp[Importer]

      Sys_Int_FOR_St[Raw Data Storage]

      Sys_Int_FOR_AggSt[Aggregated Data Storage]

      Sys_Int_FOR_Agg[Aggregation]

      Sys_Int_FOR_TaxRl[Tax rules]

      Sys_Int_FOR_Rep[Reporting]

      Sys_Int_FOR_Imp -->|transforms data\ninto an internal model| Sys_Int_FOR_St

      Sys_Int_FOR_St -->|provides operations data| Sys_Int_FOR_Agg
      
      Sys_Int_FOR_AggSt -->|provides aggregatred data| Sys_Int_FOR_Rep

      Sys_Int_FOR_TaxRl -.->|provides rules\nfor calculations| Sys_Int_FOR_Agg

      Sys_Int_FOR_Agg -->|stores data views| Sys_Int_FOR_AggSt
    end
  end

  Actor_TA -->|provides tax rules| Actor_SysOp
  
  Actor_SysOp -->|configures tax rules| Sys_Int_FOR_TaxRl

  Sys_Ext_TP & Sys_Ext_FP -->|extracts operations| Sys_Int_FOR_Imp

  Sys_Int_FOR_Rep -->|presents reports for verification| Actor_TA


```
