'use client';
import { Label } from './label';
import { Input } from './input';
import { cn } from '@/utils/cn';
import { Spinner } from './spinner';
import { Select } from './select';
import { SelectContent, SelectGroup, SelectItem, SelectLabel, SelectTrigger, SelectValue } from './select';
import { MouseEventHandler } from 'react';

export enum FormStatus {
	Awaiting = 'awaiting',
	Loading = 'loading',
	Message = 'message',
}

export enum QuestionKind {
	Select = 'select',
	Input = 'input',
}

export interface SelectItem {
	value: string;
	display: string;
}

export interface Question {
	key: string;
	display: string;
	kind: QuestionKind;
	as?: 'email' | 'password';
	placeholder?: string;
	options?: SelectItem[];
}

export interface FromProps {
	title: string;
	body: string | React.ReactNode;
	status: FormStatus;
	questions: Question[];
	submit: (answers: Record<string, string | undefined>) => void;
	message?: string | React.ReactNode;
	submit_text: string;
}

export function Form({
	submit,
	title,
	body,
	status,
	questions,
	message,
	submit_text,
}: FromProps) {
	const handle = (e: React.MouseEvent<HTMLButtonElement, MouseEvent>) => {
		const form = e.currentTarget.parentElement as HTMLFormElement;

		const response = {} as Record<string, string | undefined>;

		for (const question of questions) {
			switch (question.kind) {
				case QuestionKind.Input: {
					const v = (form.elements.namedItem(question.key) as HTMLInputElement).value;
					response[question.key] = v;
					break;
				}
				case QuestionKind.Select: {
					const v = (form.elements.namedItem(question.key) as HTMLSelectElement).value;
					response[question.key] = v;
					break;
				}
			}
		}

		submit(response);
	};

	return (
		<div className={ 'max-w-md w-96 mx-auto rounded-sm md:rounded-2xl pt-4 px-4 shadow-input bg-white dark:bg-black' + (status == FormStatus.Awaiting ? '' : ' pb-4') }>
			{(() => {
				switch (status) {
					case FormStatus.Awaiting: {
						return (
							<>
								<h2 className='font-bold text-xl text-neutral-800 dark:text-neutral-200'>
									{ title }
								</h2>
								<p className='text-neutral-600 text-sm max-w-sm mt-2 dark:text-neutral-300' style={{ marginBottom: '-1rem' }}>
									{ body }
								</p>

								<form className='my-8'>
									{questions.map((question) => {
										switch (question.kind) {
											case QuestionKind.Input: return (
												<LabelInputContainer className='mb-4' key={ question.key }>
													<Label htmlFor={ question.as }>{question.display}</Label>
													<Input name={ question.key } placeholder={ question.placeholder } type={ question.as } />
												</LabelInputContainer>
											);
											case QuestionKind.Select: return (
												<LabelInputContainer key={ question.key }>
													<Label htmlFor={ question.as }>{question.display}</Label>
													<Select name={ question.key }>
														<SelectTrigger className='w-[180px]'>
															<SelectValue placeholder='Select' />
														</SelectTrigger>
														<SelectContent>
															<SelectGroup>
																<SelectLabel>Options</SelectLabel>
																{question.options?.map(option => (
																	<SelectItem key={ option.value } value={ option.value }>
																		{ option.display }
																	</SelectItem>
																))}
															</SelectGroup>
														</SelectContent>
													</Select>
												</LabelInputContainer>
											);
										}
									})}

									<button
										className='mt-8 bg-gradient-to-br relative group/btn from-black dark:from-zinc-900 dark:to-zinc-900 to-neutral-600 block dark:bg-zinc-800 w-full text-white rounded-md h-10 font-medium shadow-[0px_1px_0px_0px_#ffffff40_inset,0px_-1px_0px_0px_#ffffff40_inset] dark:shadow-[0px_1px_0px_0px_var(--zinc-800)_inset,0px_-1px_0px_0px_var(--zinc-800)_inset]'
										onClick={ handle }
									>
										{submit_text}
										&nbsp;&nbsp;&rarr;
										<BottomGradient />
									</button>

									<div className='bg-gradient-to-r from-transparent via-neutral-300 dark:via-neutral-700 to-transparent mt-4 h-[1px] w-full' />
								</form>
							</>
						);
					}
					case FormStatus.Loading: {
						return (
							<div className='m-auto'>
								<Spinner />
							</div>
						);
					}
					case FormStatus.Message: {
						return (
							<div className='m-auto font-bold text-sm text-neutral-800 dark:text-neutral-200 self-begin'>
								{ message ?? 'Failure' }
							</div>
						);
					}
				}
			})()}
		</div>
	);
}

const BottomGradient = () => {
	return (
		<>
			<span className='group-hover/btn:opacity-100 block transition duration-500 opacity-0 absolute h-px w-full -bottom-px inset-x-0 bg-gradient-to-r from-transparent via-cyan-500 to-transparent' />
			<span className='group-hover/btn:opacity-100 blur-sm block transition duration-500 opacity-0 absolute h-px w-1/2 mx-auto -bottom-px inset-x-10 bg-gradient-to-r from-transparent via-indigo-500 to-transparent' />
		</>
	);
};

const LabelInputContainer = ({
	children,
	className,
}: {
	children: React.ReactNode;
	className?: string;
}) => {
	return (
		<div className={ cn('flex flex-col space-y-2 w-full', className) }>
			{children}
		</div>
	);
};
