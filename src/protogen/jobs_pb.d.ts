import * as auth_pb from './auth_pb'; // proto import: "auth.proto"

export interface Job {
	id: string,
    created: string,
    claimed?: string,
    pickedup?: string,
    droppedoff?: string,
    cancelled?: string,
    status: JobStatus,
}

export interface JobId {
	job_id: string,
}

export interface JobUpdateRequest {
	job_id: string,
    auth?: auth_pb.Authorization,
}

export enum JobStatus { 
  CREATED = 0,
  CLAIMED = 1,
  PICKEDUP = 2,
  DROPPEDOFF = 3,
  CANCELLED = 4,
}
