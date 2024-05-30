import { invoke } from '@tauri-apps/api/core';
import * as auth_pb from './auth'; // proto import: "auth.proto"

export interface Job {
	id: string,

    posted: Date,
    claimed?: Date,
    pickedup?: Date,
    droppedoff?: Date,
    cancelled?: Date,

	posted_by: string,
	dropoff_to: string,
	claimed_by?: string,

    status: JobStatus,
}

interface RawJob {
	id: string,
	created: string,
	claimed?: string,
	pickedup?: string,
	droppedoff?: string,
	cancelled?: string,
	posted_by: string,
	dropoff_to: string,
	claimed_by?: string,
	status: 0 | 1 | 2 | 3 | 4,
}

function date(it: string | undefined): Date | undefined {
	console.log(it, new Date(it!));
	
	if (!it) return undefined;
	else return new Date(it);
}

function convert(raw: RawJob): Job {
	return {
		...raw,
		posted: date(raw.created)!,
		claimed: date(raw.claimed),
		pickedup: date(raw.pickedup),
		droppedoff: date(raw.droppedoff),
		cancelled: date(raw.cancelled),
	}
}

export interface JobId {
	job_id: string,
}

export interface JobUpdateRequest {
	job_id: string,
    auth?: auth_pb.Authorization,
}

export enum JobStatus { 
  Created = 0,
  Claimed = 1,
  PickedUp = 2,
  DroppedOff = 3,
  Cancelled = 4,
}

export const Jobs = {
	List: () => invoke<RawJob[]>('jobs_list', {}).then((jobs) => jobs.map(convert)),
	Get: (request: JobId) => invoke<RawJob>('jobs_get', { request }).then(convert),
	Post: (request: auth_pb.Authorization) => invoke<void>('jobs_post', { request }),
	Cancel: (request: JobUpdateRequest) => invoke<void>('jobs_cancel', { request }),
	Claim: (request: JobUpdateRequest) => invoke<void>('jobs_claim', { request }),
	Unclaim: (request: JobUpdateRequest) => invoke<void>('jobs_unclaim', { request }),
	Pickup: (request: JobUpdateRequest) => invoke<void>('jobs_pickup', { request }),
	Dropoff: (request: JobUpdateRequest) => invoke<void>('jobs_dropoff', { request }),
};