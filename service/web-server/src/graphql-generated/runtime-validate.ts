import { z } from 'zod'
import { Account, Test } from 'src/graphql-generated/graphql.ts'

type Properties<T> = Required<{
  [K in keyof T]: z.ZodType<T[K], any, T[K]>;
}>;

type definedNonNullAny = {};

export const isDefinedNonNullAny = (v: any): v is definedNonNullAny => v !== undefined && v !== null;

export const definedNonNullAnySchema = z.any().refine((v) => isDefinedNonNullAny(v));

export function AccountSchema(): z.ZodObject<Properties<Account>> {
  return z.object({
    __typename: z.literal('Account').optional(),
    createdAt: z.coerce.date(),
    id: z.string()
  })
}

export function TestSchema(): z.ZodObject<Properties<Test>> {
  return z.object({
    __typename: z.literal('Test').optional(),
    d: z.coerce.date(),
    i: z.number(),
    s: z.string()
  })
}
