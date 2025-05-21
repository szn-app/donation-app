import { z } from 'zod'
import { Account, Category, Collection, CollectionType, CollectionVisibility, Committee, CommitteeRole, Community, CommunityType, Item, ItemCondition, ItemIntentAction, ItemStatus, ItemType, Location, Media, MediaType, Message, MessageType, Pledge, PledgeIntentAction, PledgeStatus, Profile, ProfileType, Publish, Review, Schedule, ScheduleOpportunity, Test, Transaction, TransactionStatus } from './graphql.ts'

type Properties<T> = Required<{
  [K in keyof T]: z.ZodType<T[K], any, T[K]>;
}>;

type definedNonNullAny = {};

export const isDefinedNonNullAny = (v: any): v is definedNonNullAny => v !== undefined && v !== null;

export const definedNonNullAnySchema = z.any().refine((v) => isDefinedNonNullAny(v));

export const CollectionTypeSchema = z.nativeEnum(CollectionType);

export const CollectionVisibilitySchema = z.nativeEnum(CollectionVisibility);

export const CommitteeRoleSchema = z.nativeEnum(CommitteeRole);

export const CommunityTypeSchema = z.nativeEnum(CommunityType);

export const ItemConditionSchema = z.nativeEnum(ItemCondition);

export const ItemIntentActionSchema = z.nativeEnum(ItemIntentAction);

export const ItemStatusSchema = z.nativeEnum(ItemStatus);

export const ItemTypeSchema = z.nativeEnum(ItemType);

export const MediaTypeSchema = z.nativeEnum(MediaType);

export const MessageTypeSchema = z.nativeEnum(MessageType);

export const PledgeIntentActionSchema = z.nativeEnum(PledgeIntentAction);

export const PledgeStatusSchema = z.nativeEnum(PledgeStatus);

export const ProfileTypeSchema = z.nativeEnum(ProfileType);

export const TransactionStatusSchema = z.nativeEnum(TransactionStatus);

export function AccountSchema(): z.ZodObject<Properties<Account>> {
  return z.object({
    __typename: z.literal('Account').optional(),
    createdAt: z.coerce.date(),
    id: z.string(),
    remarks: z.string().nullish()
  })
}

export function CategorySchema(): z.ZodObject<Properties<Category>> {
  return z.object({
    __typename: z.literal('Category').optional(),
    categoryParent: z.number().nullish(),
    createdAt: z.coerce.date(),
    description: z.string().nullish(),
    id: z.number(),
    title: z.string(),
    updatedAt: z.coerce.date()
  })
}

export function CollectionSchema(): z.ZodObject<Properties<Collection>> {
  return z.object({
    __typename: z.literal('Collection').optional(),
    createdAt: z.coerce.date(),
    id: z.number(),
    idCommunity: z.number(),
    position: z.number(),
    title: z.string(),
    type: CollectionTypeSchema,
    updatedAt: z.coerce.date(),
    visibility: CollectionVisibilitySchema
  })
}

export function CommitteeSchema(): z.ZodObject<Properties<Committee>> {
  return z.object({
    __typename: z.literal('Committee').optional(),
    idCommunity: z.number(),
    idProfile: z.number(),
    joinedAt: z.coerce.date(),
    memberRole: CommitteeRoleSchema
  })
}

export function CommunitySchema(): z.ZodObject<Properties<Community>> {
  return z.object({
    __typename: z.literal('Community').optional(),
    createdAt: z.coerce.date(),
    createdBy: z.string(),
    description: z.string().nullish(),
    id: z.number(),
    owner: z.string(),
    title: z.string().nullish(),
    type: CommunityTypeSchema,
    updatedAt: z.coerce.date().nullish()
  })
}

export function ItemSchema(): z.ZodObject<Properties<Item>> {
  return z.object({
    __typename: z.literal('Item').optional(),
    condition: ItemConditionSchema,
    createdAt: z.coerce.date(),
    description: z.string().nullish(),
    id: z.number(),
    idCategory: z.number(),
    idLocation: z.number(),
    intentAction: ItemIntentActionSchema,
    quantity: z.number(),
    status: ItemStatusSchema,
    title: z.string(),
    type: ItemTypeSchema,
    updatedAt: z.coerce.date()
  })
}

export function LocationSchema(): z.ZodObject<Properties<Location>> {
  return z.object({
    __typename: z.literal('Location').optional(),
    addressLine1: z.string(),
    addressLine2: z.string().nullish(),
    city: z.string(),
    country: z.string(),
    createdAt: z.coerce.date(),
    district: z.string().nullish(),
    entranceNote: z.string().nullish(),
    geom: z.string().nullish(),
    id: z.number(),
    state: z.string(),
    updatedAt: z.coerce.date()
  })
}

export function MediaSchema(): z.ZodObject<Properties<Media>> {
  return z.object({
    __typename: z.literal('Media').optional(),
    caption: z.string().nullish(),
    createdAt: z.coerce.date(),
    id: z.number(),
    idItem: z.number(),
    type: MediaTypeSchema,
    updatedAt: z.coerce.date(),
    url: z.string()
  })
}

export function MessageSchema(): z.ZodObject<Properties<Message>> {
  return z.object({
    __typename: z.literal('Message').optional(),
    content: z.string(),
    createdAt: z.coerce.date(),
    id: z.number(),
    idSender: z.string(),
    idTransaction: z.number(),
    type: MessageTypeSchema,
    updatedAt: z.coerce.date()
  })
}

export function PledgeSchema(): z.ZodObject<Properties<Pledge>> {
  return z.object({
    __typename: z.literal('Pledge').optional(),
    createdAt: z.coerce.date(),
    id: z.number(),
    idItem: z.number(),
    idProfile: z.string(),
    intentAction: PledgeIntentActionSchema,
    message: z.string().nullish(),
    status: PledgeStatusSchema,
    updatedAt: z.coerce.date()
  })
}

export function ProfileSchema(): z.ZodObject<Properties<Profile>> {
  return z.object({
    __typename: z.literal('Profile').optional(),
    createdAt: z.coerce.date(),
    createdBy: z.string(),
    description: z.string().nullish(),
    id: z.number(),
    name: z.string().nullish(),
    owner: z.string(),
    type: ProfileTypeSchema.nullish(),
    updatedAt: z.coerce.date().nullish()
  })
}

export function PublishSchema(): z.ZodObject<Properties<Publish>> {
  return z.object({
    __typename: z.literal('Publish').optional(),
    addedBy: z.string(),
    createdAt: z.coerce.date(),
    idCollection: z.number(),
    idItem: z.number(),
    note: z.string().nullish(),
    position: z.number(),
    updatedAt: z.coerce.date()
  })
}

export function ReviewSchema(): z.ZodObject<Properties<Review>> {
  return z.object({
    __typename: z.literal('Review').optional(),
    comment: z.string().nullish(),
    createdAt: z.coerce.date(),
    idSubjectProfile: z.string(),
    idTransaction: z.number(),
    rating: z.number(),
    updatedAt: z.coerce.date()
  })
}

export function ScheduleSchema(): z.ZodObject<Properties<Schedule>> {
  return z.object({
    __typename: z.literal('Schedule').optional(),
    createdAt: z.coerce.date(),
    id: z.number(),
    scheduledFor: z.coerce.date(),
    updatedAt: z.coerce.date()
  })
}

export function ScheduleOpportunitySchema(): z.ZodObject<Properties<ScheduleOpportunity>> {
  return z.object({
    __typename: z.literal('ScheduleOpportunity').optional(),
    createdAt: z.coerce.date(),
    id: z.number(),
    updatedAt: z.coerce.date(),
    windowEnd: z.coerce.date(),
    windowStart: z.coerce.date()
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

export function TransactionSchema(): z.ZodObject<Properties<Transaction>> {
  return z.object({
    __typename: z.literal('Transaction').optional(),
    createdAt: z.coerce.date(),
    id: z.number(),
    idLocation: z.number().nullish(),
    idPledge: z.number(),
    idSchedule: z.number().nullish(),
    status: TransactionStatusSchema,
    updatedAt: z.coerce.date()
  })
}
