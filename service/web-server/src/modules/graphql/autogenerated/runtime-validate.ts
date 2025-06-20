import { z } from 'zod'
import { Account, Category, Collection, CollectionType, CollectionVisibility, Committee, CommitteeRole, Community, CommunityType, Coordinates, CoordinatesInput, Item, ItemCondition, ItemIntentAction, ItemStatus, ItemType, Location, Media, MediaType, Message, MessageType, Pledge, PledgeIntentAction, PledgeStatus, Profile, ProfileType, Publish, Review, Schedule, ScheduleOpportunity, Test, Transaction, TransactionStatus } from './graphql.ts'

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
    description: z.string().nullish(),
    id: z.number(),
    title: z.string()
  })
}

export function CollectionSchema(): z.ZodObject<Properties<Collection>> {
  return z.object({
    __typename: z.literal('Collection').optional(),
    createdAt: z.coerce.date(),
    id: z.number(),
    idCommunity: z.number().nullish(),
    position: z.number(),
    title: z.string().nullish(),
    updatedAt: z.coerce.date().nullish(),
    variant: CollectionTypeSchema.nullish(),
    visibility: CollectionVisibilitySchema
  })
}

export function CommitteeSchema(): z.ZodObject<Properties<Committee>> {
  return z.object({
    __typename: z.literal('Committee').optional(),
    idCommunity: z.number(),
    idProfile: z.number(),
    joined_at: z.coerce.date(),
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
    title: z.string(),
    updatedAt: z.coerce.date().nullish(),
    variant: CommunityTypeSchema
  })
}

export function CoordinatesSchema(): z.ZodObject<Properties<Coordinates>> {
  return z.object({
    __typename: z.literal('Coordinates').optional(),
    latitude: z.number(),
    longitude: z.number()
  })
}

export function CoordinatesInputSchema(): z.ZodObject<Properties<CoordinatesInput>> {
  return z.object({
    latitude: z.number(),
    longitude: z.number()
  })
}

export function ItemSchema(): z.ZodObject<Properties<Item>> {
  return z.object({
    __typename: z.literal('Item').optional(),
    category: z.number().nullish(),
    condition: ItemConditionSchema.nullish(),
    createdAt: z.coerce.date(),
    createdBy: z.string().nullish(),
    description: z.string().nullish(),
    id: z.number(),
    intentAction: ItemIntentActionSchema.nullish(),
    isReported: z.boolean(),
    location: z.number().nullish(),
    status: ItemStatusSchema.nullish(),
    title: z.string().nullish(),
    updatedAt: z.coerce.date().nullish(),
    variant: ItemTypeSchema.nullish(),
    viewsCount: z.number()
  })
}

export function LocationSchema(): z.ZodObject<Properties<Location>> {
  return z.object({
    __typename: z.literal('Location').optional(),
    addressLine1: z.string(),
    addressLine2: z.string().nullish(),
    city: z.string(),
    coordinates: CoordinatesSchema().nullish(),
    country: z.string(),
    createdAt: z.coerce.date(),
    district: z.string().nullish(),
    entranceNote: z.string().nullish(),
    id: z.number(),
    state: z.string().nullish()
  })
}

export function MediaSchema(): z.ZodObject<Properties<Media>> {
  return z.object({
    __typename: z.literal('Media').optional(),
    caption: z.string().nullish(),
    createdAt: z.coerce.date(),
    id: z.number(),
    idItem: z.number(),
    url: z.string(),
    variant: MediaTypeSchema
  })
}

export function MessageSchema(): z.ZodObject<Properties<Message>> {
  return z.object({
    __typename: z.literal('Message').optional(),
    content: z.string(),
    id: z.number(),
    idSender: z.number().nullish(),
    idTransaction: z.number(),
    sent_at: z.coerce.date(),
    updatedAt: z.coerce.date().nullish(),
    variant: MessageTypeSchema.nullish()
  })
}

export function PledgeSchema(): z.ZodObject<Properties<Pledge>> {
  return z.object({
    __typename: z.literal('Pledge').optional(),
    id: z.number(),
    idItem: z.number(),
    idProfile: z.number(),
    intentAction: PledgeIntentActionSchema,
    message: z.string().nullish(),
    pledged_at: z.coerce.date(),
    status: PledgeStatusSchema,
    updatedAt: z.coerce.date().nullish(),
    updatedBy: z.string().nullish()
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
    updatedAt: z.coerce.date().nullish(),
    variant: ProfileTypeSchema.nullish()
  })
}

export function PublishSchema(): z.ZodObject<Properties<Publish>> {
  return z.object({
    __typename: z.literal('Publish').optional(),
    addedBy: z.string().nullish(),
    idCollection: z.number(),
    idItem: z.number(),
    note: z.string().nullish(),
    position: z.number(),
    posted_on: z.coerce.date()
  })
}

export function ReviewSchema(): z.ZodObject<Properties<Review>> {
  return z.object({
    __typename: z.literal('Review').optional(),
    comment: z.string().nullish(),
    createdAt: z.coerce.date(),
    idSubjectProfile: z.number(),
    idTransaction: z.number(),
    reviewer: z.number(),
    score: z.number()
  })
}

export function ScheduleSchema(): z.ZodObject<Properties<Schedule>> {
  return z.object({
    __typename: z.literal('Schedule').optional(),
    id: z.number(),
    scheduled_for: z.coerce.date()
  })
}

export function ScheduleOpportunitySchema(): z.ZodObject<Properties<ScheduleOpportunity>> {
  return z.object({
    __typename: z.literal('ScheduleOpportunity').optional(),
    id: z.number(),
    windowEnd: z.coerce.date().nullish(),
    windowStart: z.coerce.date().nullish()
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
    updatedAt: z.coerce.date().nullish()
  })
}
