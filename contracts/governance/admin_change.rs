 
// Astroport Governance Admin Change Contract

use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, WasmMsg, StdResult, to_binary, Uint128};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ProposalType {
    ParameterChange,
    CommunitySpend,
    ContractMigration,
    ChangeAdmin,  // NEW: Change contract admin proposal
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Proposal {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub proposal_type: ProposalType,
    pub target_contract: Option<String>,  // Target contract for admin change
    pub new_admin: Option<String>,  // New admin address
    pub status: ProposalStatus,
    pub yes_votes: Uint128,
    pub no_votes: Uint128,
    pub end_time: u64,
}

pub fn execute_proposal(deps: DepsMut, env: Env, info: MessageInfo, proposal_id: u64) -> Result<Response, ContractError> {
    let mut proposal = PROPOSALS.load(deps.storage, proposal_id)?;

    if env.block.time.seconds() < proposal.end_time {
        return Err(ContractError::VotingPeriodNotOver {});
    }

    if proposal.status != ProposalStatus::Passed {
        return Err(ContractError::ProposalDidNotPass {});
    }

    if proposal.proposal_type == ProposalType::ChangeAdmin {
        if let (Some(contract), Some(new_admin)) = (proposal.target_contract.clone(), proposal.new_admin.clone()) {
            let admin_update_msg = WasmMsg::Execute {
                contract_addr: contract.clone(),
                msg: to_binary(&UpdateAdminMsg { new_admin })?,
                funds: vec![],
            };

            proposal.status = ProposalStatus::Executed;
            PROPOSALS.save(deps.storage, proposal_id, &proposal)?;

            return Ok(Response::new()
                .add_message(admin_update_msg)
                .add_attribute("action", "change_admin")
                .add_attribute("contract", contract)
                .add_attribute("new_admin", new_admin));
        }
    }

    Err(ContractError::InvalidProposal {})
}
