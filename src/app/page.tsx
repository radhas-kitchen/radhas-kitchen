'use client';

import { Button } from '@components/ui/button';
import { Spinner } from '@components/ui/spinner';
import { LoginResponse, UserKind } from '@proto/auth';
import { Job, JobStatus, Jobs } from '@proto/jobs';
import { ensureToken, useAuth } from '@utils/auth';
import { useState } from 'react';
import { useAsync } from '@utils/async';

const Filters = {
	[UserKind.Provider]: (j: Job, uid: string) => j.posted_by == uid,
	[UserKind.Driver]: (j: Job, uid: string) => (j.status == JobStatus.Created || j.claimed_by == uid) && j.status != JobStatus.DroppedOff,
	[UserKind.Consumer]: (j: Job, uid: string) => j.dropoff_to == uid,
};

interface Hooks {
	reload: () => void;
	setLoading: (_: boolean) => void;
	setError: (_: Error) => void;
}

interface JobDefaultProps {
	job: Job;
	children?: React.ReactNode;
}

function JobDefault({ job, children }: JobDefaultProps) {
	return (
		<div className='w-full bg-black rounded-md pt-4 px-4 flex flex-col gap-4' key={ job.id }>
			<div className='flex justify-between'>
				<p>
					Job, created at&nbsp;
					{job.posted.toLocaleString().trim()}
				</p>
				<p>
					Status:&nbsp;
					{ JobStatus[job.status] }
				</p>
			</div>

			{ children }
			<div className='bg-gradient-to-r from-transparent via-neutral-300 dark:via-neutral-700 to-transparent h-[1px] w-full' />
		</div>
	);
}

function ProviderJob(job: Job, user: LoginResponse, {
	reload, setLoading, setError,
}: Hooks) {
	return (
		<JobDefault job={ job }>
			{ job.status < JobStatus.PickedUp && (
				<Button
					className='w-full'
					onClick={ () => {
						setLoading(true);

						Jobs.Cancel({
							job_id: job.id,
							auth: { ...user },
						}).then(reload).catch((e) => {
							setError(e);
							setLoading(false);
						});
					} }
				>
					Cancel
				</Button>
			) }
		</JobDefault>
	);
}

function DriverJob(job: Job, user: LoginResponse, {
	reload, setLoading, setError,
}: Hooks) {
	return (
		<JobDefault job={ job }>
			{(() => {
				switch (job.status) {
					case JobStatus.Created: return (
						<Button
							className='w-full'
							onClick={ () => {
								setLoading(true);

								Jobs.Claim({
									job_id: job.id,
									auth: { ...user },
								}).then(reload).catch((e) => {
									setError(e);
									setLoading(false);
								});
							} }
						>
							Claim
						</Button>
					);
					case JobStatus.Claimed: return (
						<>
							<Button
								className='w-full'
								onClick={ () => {
									setLoading(true);

									Jobs.Pickup({
										job_id: job.id,
										auth: { ...user },
									}).then(reload).catch((e) => {
										setError(e);
										setLoading(false);
									});
								} }
							>
								Mark Picked Up
							</Button>

							<Button
								className='w-full'
								onClick={ () => {
									setLoading(true);

									Jobs.Unclaim({
										job_id: job.id,
										auth: { ...user },
									}).then(reload).catch((e) => {
										setError(e);
										setLoading(false);
									});
								} }
							>
								Unclaim
							</Button>
						</>
					);
					case JobStatus.PickedUp: return (
						<Button
							className='w-full'
							onClick={ () => {
								setLoading(true);

								Jobs.Dropoff({
									job_id: job.id,
									auth: { ...user },
								}).then(reload).catch((e) => {
									setError(e);
									setLoading(false);
								});
							} }
						>
							Mark Dropped Off
						</Button>
					);
					default: throw new Error('Invalid State');
				}
			})()}
		</JobDefault>
	);
}

function ConsumerJob(job: Job) {
	return (
		<JobDefault job={ job } />
	);
}

const JobElems = {
	[UserKind.Provider]: ProviderJob,
	[UserKind.Driver]: DriverJob,
	[UserKind.Consumer]: (job: Job, _: LoginResponse, __: Hooks) => ConsumerJob(job),
};

export default function Index() {
	const auth = useAuth();
	const user = ensureToken(auth);
	const [loading, setLoading] = useState(true);
	const [error, setError] = useState<Error | undefined>(undefined);
	const [jobs, reload] = useAsync(() => Jobs.List().then(j => j.filter((j) => {
		if (!user) return true;
		else return Filters[user.kind](j, user.user_id);
	})), setLoading, setError);

	if (error) return (
		<div className='w-full h-full content-center'>
			<div className='m-auto'>
				<h1 className='text-center text-lg'>Error</h1>
				<p className='text-center text-sm text-zinc-300'>{ error.message }</p>
				<Button className='w-full mt-4' onClick={ () => reload() }>
					Try Again
				</Button>
			</div>
		</div>
	);

	if (loading || !user || !jobs) return (
		<div className='w-full h-full content-center'>
			<div className='m-auto'>
				<Spinner />
			</div>
		</div>
	);

	return (
		<div className='w-full h-full p-4 content-start flex flex-col justify-between'>
			<div className='w-full content-start flex flex-col gap-4'>
				{ (() => {
					if (user.kind != UserKind.Provider) return <></>;

					return (
						<Button
							className='w-full'
							onClick={ () => {
								setLoading(true);
								Jobs.Post({ ...user }).then(reload).catch((e) => {
									setError(e);
									setLoading(false);
								});
							} }
						>
							Create Job
						</Button>
					);
				})() }

				{ (() => {
					if (jobs.length == 0) return <div className='m-auto'>No Jobs Listed</div>;
					else return jobs.map(job => JobElems[user.kind](job, user, { reload, setLoading, setError }));
				})() }
			</div>

			<div>
				<Button className='mt-4 w-full' onClick={ () => reload() }>
					Reload
				</Button>

				<Button className='mt-4 w-full' onClick={ () => auth.logout() }>
					Sign Out
				</Button>
			</div>
		</div>
	);
}
