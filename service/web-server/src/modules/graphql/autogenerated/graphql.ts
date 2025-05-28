/* eslint-disable */
import type { DocumentTypeDecoration } from '@graphql-typed-document-node/core';
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
export type MakeEmpty<T extends { [key: string]: unknown }, K extends keyof T> = { [_ in K]?: never };
export type Incremental<T> = T | { [P in keyof T]?: P extends ' $fragmentName' | '__typename' ? T[P] : never };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: { input: string; output: string; }
  String: { input: string; output: string; }
  Boolean: { input: boolean; output: boolean; }
  Int: { input: number; output: number; }
  Float: { input: number; output: number; }
  /**
   * A datetime with timezone offset.
   *
   * The input is a string in RFC3339 format, e.g. "2022-01-12T04:00:19.12345Z"
   * or "2022-01-12T04:00:19+03:00". The output is also a string in RFC3339
   * format, but it is always normalized to the UTC (Z) offset, e.g.
   * "2022-01-12T04:00:19.12345Z".
   */
  DateTime: { input: string; output: Date; }
  /**
   * A UUID is a unique 128-bit number, stored as 16 octets. UUIDs are parsed as
   * Strings within GraphQL. UUIDs are used to assign unique identifiers to
   * entities without requiring a central allocating authority.
   *
   * # References
   *
   * * [Wikipedia: Universally Unique Identifier](http://en.wikipedia.org/wiki/Universally_unique_identifier)
   * * [RFC4122: A Universally Unique Identifier (UUID) URN Namespace](http://tools.ietf.org/html/rfc4122)
   */
  UUID: { input: string; output: string; }
};

export type Account = {
  __typename?: 'Account';
  createdAt: Scalars['DateTime']['output'];
  id: Scalars['UUID']['output'];
  remarks?: Maybe<Scalars['String']['output']>;
};

export type Category = {
  __typename?: 'Category';
  categoryParent?: Maybe<Scalars['Int']['output']>;
  description?: Maybe<Scalars['String']['output']>;
  id: Scalars['Int']['output'];
  title: Scalars['String']['output'];
};

export type Collection = {
  __typename?: 'Collection';
  createdAt: Scalars['DateTime']['output'];
  id: Scalars['Int']['output'];
  idCommunity?: Maybe<Scalars['Int']['output']>;
  position: Scalars['Int']['output'];
  title?: Maybe<Scalars['String']['output']>;
  updatedAt?: Maybe<Scalars['DateTime']['output']>;
  variant?: Maybe<CollectionType>;
  visibility: CollectionVisibility;
};

export enum CollectionType {
  Featured = 'featured',
  Regular = 'regular'
}

export enum CollectionVisibility {
  Public = 'public',
  Restricted = 'restricted'
}

export type Committee = {
  __typename?: 'Committee';
  idCommunity: Scalars['Int']['output'];
  idProfile: Scalars['Int']['output'];
  joined_at: Scalars['DateTime']['output'];
  memberRole: CommitteeRole;
};

export enum CommitteeRole {
  Member = 'member',
  Organizer = 'organizer'
}

export type Community = {
  __typename?: 'Community';
  createdAt: Scalars['DateTime']['output'];
  createdBy: Scalars['UUID']['output'];
  description?: Maybe<Scalars['String']['output']>;
  id: Scalars['Int']['output'];
  owner: Scalars['UUID']['output'];
  title: Scalars['String']['output'];
  updatedAt?: Maybe<Scalars['DateTime']['output']>;
  variant: CommunityType;
};

export enum CommunityType {
  Organized = 'organized',
  Solo = 'solo'
}

/**
 * Convertion flow:
 * - CoordinatesInput -> GeoPoint -> PostGisPoint -> Database
 * - Database -> PostGisPoint -> GeoPoint -> Coordinates (for GraphQL)
 */
export type Coordinates = {
  __typename?: 'Coordinates';
  latitude: Scalars['Float']['output'];
  longitude: Scalars['Float']['output'];
};

export type CoordinatesInput = {
  latitude: Scalars['Float']['input'];
  longitude: Scalars['Float']['input'];
};

export type Item = {
  __typename?: 'Item';
  category?: Maybe<Scalars['Int']['output']>;
  condition?: Maybe<ItemCondition>;
  createdAt: Scalars['DateTime']['output'];
  createdBy?: Maybe<Scalars['UUID']['output']>;
  description?: Maybe<Scalars['String']['output']>;
  id: Scalars['Int']['output'];
  intentAction: ItemIntentAction;
  isReported: Scalars['Boolean']['output'];
  location?: Maybe<Scalars['Int']['output']>;
  status?: Maybe<ItemStatus>;
  title?: Maybe<Scalars['String']['output']>;
  updatedAt?: Maybe<Scalars['DateTime']['output']>;
  variant: ItemType;
  viewsCount: Scalars['Int']['output'];
};

export enum ItemCondition {
  BrandNew = 'brand_new',
  PreOwnedBarelyUsed = 'pre_owned_barely_used',
  PreOwnedDamaged = 'pre_owned_damaged',
  PreOwnedUsable = 'pre_owned_usable'
}

export enum ItemIntentAction {
  Offer = 'offer',
  Request = 'request'
}

export enum ItemStatus {
  Active = 'active',
  Archived = 'archived',
  Disabled = 'disabled',
  Draft = 'draft'
}

export enum ItemType {
  Inkind = 'inkind',
  Inquiry = 'inquiry',
  Monetary = 'monetary',
  Service = 'service'
}

export type Location = {
  __typename?: 'Location';
  addressLine1: Scalars['String']['output'];
  addressLine2?: Maybe<Scalars['String']['output']>;
  city: Scalars['String']['output'];
  coordinates?: Maybe<Coordinates>;
  country: Scalars['String']['output'];
  createdAt: Scalars['DateTime']['output'];
  district?: Maybe<Scalars['String']['output']>;
  entranceNote?: Maybe<Scalars['String']['output']>;
  id: Scalars['Int']['output'];
  state?: Maybe<Scalars['String']['output']>;
};

export type Media = {
  __typename?: 'Media';
  caption?: Maybe<Scalars['String']['output']>;
  createdAt: Scalars['DateTime']['output'];
  id: Scalars['Int']['output'];
  idItem: Scalars['Int']['output'];
  url: Scalars['String']['output'];
  variant: MediaType;
};

export enum MediaType {
  Document = 'document',
  Image = 'image',
  Video = 'video'
}

export type Message = {
  __typename?: 'Message';
  content: Scalars['String']['output'];
  id: Scalars['Int']['output'];
  idSender?: Maybe<Scalars['Int']['output']>;
  idTransaction: Scalars['Int']['output'];
  sent_at: Scalars['DateTime']['output'];
  updatedAt?: Maybe<Scalars['DateTime']['output']>;
  variant?: Maybe<MessageType>;
};

export enum MessageType {
  ScheduleOpportunity = 'schedule_opportunity',
  Text = 'text'
}

/** GraphQL Mutation Root */
export type Mutation = {
  __typename?: 'Mutation';
  create: Message;
  createAccount: Account;
  createCategory: Category;
  createCollection: Collection;
  createCommittee: Committee;
  createCommunity: Community;
  createItem: Item;
  createLocation: Location;
  createMedia: Media;
  createProfile: Profile;
  createPublish: Publish;
  deleteAccount: Scalars['Boolean']['output'];
  deleteCategory: Scalars['Boolean']['output'];
  deleteCollection: Scalars['Boolean']['output'];
  deleteCommittee: Scalars['Boolean']['output'];
  deleteCommunity: Scalars['Boolean']['output'];
  deleteItem: Scalars['Boolean']['output'];
  deleteLocation: Scalars['Boolean']['output'];
  deleteMedia: Scalars['Boolean']['output'];
  deleteProfile: Scalars['Boolean']['output'];
  deletePublish: Scalars['Boolean']['output'];
  reportItem?: Maybe<Item>;
  update: Transaction;
  updateAccount?: Maybe<Account>;
  updateCategory: Category;
  updateCollection: Collection;
  updateCommittee?: Maybe<Committee>;
  updateCommunity?: Maybe<Community>;
  updateItem: Item;
  updateLocation: Location;
  updateMedia: Media;
  updateProfile?: Maybe<Profile>;
  updatePublish: Publish;
};


/** GraphQL Mutation Root */
export type MutationCreateArgs = {
  content: Scalars['String']['input'];
  idSender: Scalars['UUID']['input'];
  idTransaction: Scalars['Int']['input'];
  variant: MessageType;
};


/** GraphQL Mutation Root */
export type MutationCreateAccountArgs = {
  id: Scalars['UUID']['input'];
  remarks?: InputMaybe<Scalars['String']['input']>;
};


/** GraphQL Mutation Root */
export type MutationCreateCategoryArgs = {
  description: Scalars['String']['input'];
  name: Scalars['String']['input'];
  parentId?: InputMaybe<Scalars['Int']['input']>;
};


/** GraphQL Mutation Root */
export type MutationCreateCollectionArgs = {
  idCommunity: Scalars['Int']['input'];
  position: Scalars['Int']['input'];
  title: Scalars['String']['input'];
  variant: CollectionType;
  visibility: CollectionVisibility;
};


/** GraphQL Mutation Root */
export type MutationCreateCommitteeArgs = {
  idCommunity: Scalars['Int']['input'];
  idProfile: Scalars['Int']['input'];
  memberRole: CommitteeRole;
};


/** GraphQL Mutation Root */
export type MutationCreateCommunityArgs = {
  createdBy: Scalars['UUID']['input'];
  description?: InputMaybe<Scalars['String']['input']>;
  name: Scalars['String']['input'];
  variant: CommunityType;
};


/** GraphQL Mutation Root */
export type MutationCreateItemArgs = {
  category?: InputMaybe<Scalars['Int']['input']>;
  condition: ItemCondition;
  createdBy?: InputMaybe<Scalars['UUID']['input']>;
  description?: InputMaybe<Scalars['String']['input']>;
  intentAction: ItemIntentAction;
  location?: InputMaybe<Scalars['Int']['input']>;
  status?: InputMaybe<ItemStatus>;
  title?: InputMaybe<Scalars['String']['input']>;
  variant: ItemType;
};


/** GraphQL Mutation Root */
export type MutationCreateLocationArgs = {
  addressLine1: Scalars['String']['input'];
  addressLine2?: InputMaybe<Scalars['String']['input']>;
  city: Scalars['String']['input'];
  coordinates?: InputMaybe<CoordinatesInput>;
  country: Scalars['String']['input'];
  district?: InputMaybe<Scalars['String']['input']>;
  entranceNote?: InputMaybe<Scalars['String']['input']>;
  state?: InputMaybe<Scalars['String']['input']>;
};


/** GraphQL Mutation Root */
export type MutationCreateMediaArgs = {
  idItem: Scalars['Int']['input'];
  mediaType: MediaType;
  position: Scalars['Int']['input'];
  url: Scalars['String']['input'];
};


/** GraphQL Mutation Root */
export type MutationCreateProfileArgs = {
  description?: InputMaybe<Scalars['String']['input']>;
  idAccount: Scalars['UUID']['input'];
  name: Scalars['String']['input'];
};


/** GraphQL Mutation Root */
export type MutationCreatePublishArgs = {
  createdBy: Scalars['UUID']['input'];
  idCollection: Scalars['Int']['input'];
  idItem: Scalars['Int']['input'];
  note?: InputMaybe<Scalars['String']['input']>;
  position: Scalars['Int']['input'];
};


/** GraphQL Mutation Root */
export type MutationDeleteAccountArgs = {
  id: Scalars['UUID']['input'];
};


/** GraphQL Mutation Root */
export type MutationDeleteCategoryArgs = {
  id: Scalars['Int']['input'];
};


/** GraphQL Mutation Root */
export type MutationDeleteCollectionArgs = {
  id: Scalars['Int']['input'];
};


/** GraphQL Mutation Root */
export type MutationDeleteCommitteeArgs = {
  idCommunity: Scalars['Int']['input'];
  idProfile: Scalars['Int']['input'];
};


/** GraphQL Mutation Root */
export type MutationDeleteCommunityArgs = {
  id: Scalars['Int']['input'];
};


/** GraphQL Mutation Root */
export type MutationDeleteItemArgs = {
  id: Scalars['Int']['input'];
};


/** GraphQL Mutation Root */
export type MutationDeleteLocationArgs = {
  id: Scalars['Int']['input'];
};


/** GraphQL Mutation Root */
export type MutationDeleteMediaArgs = {
  id: Scalars['Int']['input'];
};


/** GraphQL Mutation Root */
export type MutationDeleteProfileArgs = {
  id: Scalars['Int']['input'];
};


/** GraphQL Mutation Root */
export type MutationDeletePublishArgs = {
  idCollection: Scalars['Int']['input'];
  idItem: Scalars['Int']['input'];
};


/** GraphQL Mutation Root */
export type MutationReportItemArgs = {
  id: Scalars['Int']['input'];
};


/** GraphQL Mutation Root */
export type MutationUpdateArgs = {
  id: Scalars['Int']['input'];
  status: TransactionStatus;
};


/** GraphQL Mutation Root */
export type MutationUpdateAccountArgs = {
  id: Scalars['UUID']['input'];
  remarks?: InputMaybe<Scalars['String']['input']>;
};


/** GraphQL Mutation Root */
export type MutationUpdateCategoryArgs = {
  description?: InputMaybe<Scalars['String']['input']>;
  id: Scalars['Int']['input'];
  name?: InputMaybe<Scalars['String']['input']>;
  parentId?: InputMaybe<Scalars['Int']['input']>;
};


/** GraphQL Mutation Root */
export type MutationUpdateCollectionArgs = {
  id: Scalars['Int']['input'];
  position: Scalars['Int']['input'];
  title: Scalars['String']['input'];
  variant: CollectionType;
  visibility: CollectionVisibility;
};


/** GraphQL Mutation Root */
export type MutationUpdateCommitteeArgs = {
  idCommunity: Scalars['Int']['input'];
  idProfile: Scalars['Int']['input'];
  memberRole: CommitteeRole;
};


/** GraphQL Mutation Root */
export type MutationUpdateCommunityArgs = {
  description?: InputMaybe<Scalars['String']['input']>;
  id: Scalars['Int']['input'];
  title?: InputMaybe<Scalars['String']['input']>;
  variant?: InputMaybe<CommunityType>;
};


/** GraphQL Mutation Root */
export type MutationUpdateItemArgs = {
  category?: InputMaybe<Scalars['Int']['input']>;
  condition?: InputMaybe<ItemCondition>;
  description?: InputMaybe<Scalars['String']['input']>;
  id: Scalars['Int']['input'];
  location?: InputMaybe<Scalars['Int']['input']>;
  status?: InputMaybe<ItemStatus>;
  title?: InputMaybe<Scalars['String']['input']>;
};


/** GraphQL Mutation Root */
export type MutationUpdateLocationArgs = {
  addressLine1?: InputMaybe<Scalars['String']['input']>;
  addressLine2?: InputMaybe<Scalars['String']['input']>;
  city?: InputMaybe<Scalars['String']['input']>;
  coordinates?: InputMaybe<CoordinatesInput>;
  country?: InputMaybe<Scalars['String']['input']>;
  district?: InputMaybe<Scalars['String']['input']>;
  entranceNote?: InputMaybe<Scalars['String']['input']>;
  id: Scalars['Int']['input'];
  state?: InputMaybe<Scalars['String']['input']>;
};


/** GraphQL Mutation Root */
export type MutationUpdateMediaArgs = {
  id: Scalars['Int']['input'];
  mediaType?: InputMaybe<MediaType>;
  position?: InputMaybe<Scalars['Int']['input']>;
  url?: InputMaybe<Scalars['String']['input']>;
};


/** GraphQL Mutation Root */
export type MutationUpdateProfileArgs = {
  description?: InputMaybe<Scalars['String']['input']>;
  id: Scalars['Int']['input'];
  name: Scalars['String']['input'];
  variant?: InputMaybe<ProfileType>;
};


/** GraphQL Mutation Root */
export type MutationUpdatePublishArgs = {
  idCollection: Scalars['Int']['input'];
  idItem: Scalars['Int']['input'];
  note?: InputMaybe<Scalars['String']['input']>;
  position: Scalars['Int']['input'];
};

export type Pledge = {
  __typename?: 'Pledge';
  id: Scalars['Int']['output'];
  idItem: Scalars['Int']['output'];
  idProfile: Scalars['Int']['output'];
  intentAction: PledgeIntentAction;
  message?: Maybe<Scalars['String']['output']>;
  pledged_at: Scalars['DateTime']['output'];
  status: PledgeStatus;
  updatedAt?: Maybe<Scalars['DateTime']['output']>;
  updatedBy?: Maybe<Scalars['UUID']['output']>;
};

export enum PledgeIntentAction {
  Give = 'give',
  Receive = 'receive'
}

export enum PledgeStatus {
  Approved = 'approved',
  Declined = 'declined',
  Pending = 'pending'
}

export type Profile = {
  __typename?: 'Profile';
  createdAt: Scalars['DateTime']['output'];
  createdBy: Scalars['UUID']['output'];
  description?: Maybe<Scalars['String']['output']>;
  id: Scalars['Int']['output'];
  name?: Maybe<Scalars['String']['output']>;
  owner: Scalars['UUID']['output'];
  updatedAt?: Maybe<Scalars['DateTime']['output']>;
  variant?: Maybe<ProfileType>;
};

export enum ProfileType {
  Company = 'company',
  Individual = 'individual',
  Organization = 'organization'
}

export type Publish = {
  __typename?: 'Publish';
  addedBy?: Maybe<Scalars['UUID']['output']>;
  idCollection: Scalars['Int']['output'];
  idItem: Scalars['Int']['output'];
  note?: Maybe<Scalars['String']['output']>;
  position: Scalars['Int']['output'];
  posted_on: Scalars['DateTime']['output'];
};

/** GraphQL Query Root */
export type Query = {
  __typename?: 'Query';
  dummyTest: Array<Scalars['String']['output']>;
  dummyTestRequestHeader: Scalars['String']['output'];
  dummyTestSecure: Test;
  dummyTestSecureGuard: Test;
  dummyTestSecurePermissionCheck: Test;
  /** Get account by ID */
  findAccount?: Maybe<Account>;
  findCategory?: Maybe<Category>;
  findCollection?: Maybe<Collection>;
  /** Get committee by ID */
  findCommittee?: Maybe<Committee>;
  /** Get community by ID */
  findCommunity?: Maybe<Community>;
  findItem?: Maybe<Item>;
  findLocation?: Maybe<Location>;
  findMedia?: Maybe<Media>;
  findMessage?: Maybe<Message>;
  findPledge?: Maybe<Pledge>;
  /** Get profile by ID */
  findProfile?: Maybe<Profile>;
  findPublish?: Maybe<Publish>;
  findSchedule?: Maybe<Schedule>;
  findScheduleOpportunity?: Maybe<ScheduleOpportunity>;
  findTransaction?: Maybe<Transaction>;
  /** Get all accounts */
  listAccounts: Array<Account>;
  listCategories: Array<Category>;
  listCollections: Array<Collection>;
  /** Get all committees */
  listCommittees: Array<Committee>;
  /** Get all communities */
  listCommunities: Array<Community>;
  listItems: Array<Item>;
  listLocations: Array<Location>;
  listMedia: Array<Media>;
  listMessages: Array<Message>;
  listPledges: Array<Pledge>;
  /** Get all profiles */
  listProfiles: Array<Profile>;
  listPublishes: Array<Publish>;
  listReviews: Array<Review>;
  listScheduleOpportunities: Array<ScheduleOpportunity>;
  listSchedules: Array<Schedule>;
  listTests: Array<Test>;
  listTransactions: Array<Transaction>;
};


/** GraphQL Query Root */
export type QueryFindAccountArgs = {
  id: Scalars['UUID']['input'];
};


/** GraphQL Query Root */
export type QueryFindCategoryArgs = {
  id: Scalars['Int']['input'];
};


/** GraphQL Query Root */
export type QueryFindCollectionArgs = {
  id: Scalars['Int']['input'];
};


/** GraphQL Query Root */
export type QueryFindCommitteeArgs = {
  id: Scalars['UUID']['input'];
};


/** GraphQL Query Root */
export type QueryFindCommunityArgs = {
  id: Scalars['Int']['input'];
};


/** GraphQL Query Root */
export type QueryFindItemArgs = {
  id: Scalars['Int']['input'];
};


/** GraphQL Query Root */
export type QueryFindLocationArgs = {
  id: Scalars['Int']['input'];
};


/** GraphQL Query Root */
export type QueryFindMediaArgs = {
  id: Scalars['Int']['input'];
};


/** GraphQL Query Root */
export type QueryFindMessageArgs = {
  id: Scalars['Int']['input'];
};


/** GraphQL Query Root */
export type QueryFindPledgeArgs = {
  id: Scalars['Int']['input'];
};


/** GraphQL Query Root */
export type QueryFindProfileArgs = {
  id: Scalars['Int']['input'];
};


/** GraphQL Query Root */
export type QueryFindPublishArgs = {
  id: Scalars['Int']['input'];
};


/** GraphQL Query Root */
export type QueryFindScheduleArgs = {
  id: Scalars['Int']['input'];
};


/** GraphQL Query Root */
export type QueryFindScheduleOpportunityArgs = {
  id: Scalars['Int']['input'];
};


/** GraphQL Query Root */
export type QueryFindTransactionArgs = {
  id: Scalars['Int']['input'];
};


/** GraphQL Query Root */
export type QueryListItemsArgs = {
  status?: InputMaybe<ItemStatus>;
};

export type Review = {
  __typename?: 'Review';
  comment?: Maybe<Scalars['String']['output']>;
  createdAt: Scalars['DateTime']['output'];
  idSubjectProfile: Scalars['Int']['output'];
  idTransaction: Scalars['Int']['output'];
  reviewer: Scalars['Int']['output'];
  score: Scalars['Int']['output'];
};

export type Schedule = {
  __typename?: 'Schedule';
  id: Scalars['Int']['output'];
  scheduled_for: Scalars['DateTime']['output'];
};

export type ScheduleOpportunity = {
  __typename?: 'ScheduleOpportunity';
  id: Scalars['Int']['output'];
  windowEnd?: Maybe<Scalars['DateTime']['output']>;
  windowStart?: Maybe<Scalars['DateTime']['output']>;
};

export type Test = {
  __typename?: 'Test';
  d: Scalars['DateTime']['output'];
  i: Scalars['Int']['output'];
  s: Scalars['String']['output'];
};

export type Transaction = {
  __typename?: 'Transaction';
  createdAt: Scalars['DateTime']['output'];
  id: Scalars['Int']['output'];
  idLocation?: Maybe<Scalars['Int']['output']>;
  idPledge: Scalars['Int']['output'];
  idSchedule?: Maybe<Scalars['Int']['output']>;
  status: TransactionStatus;
  updatedAt?: Maybe<Scalars['DateTime']['output']>;
};

export enum TransactionStatus {
  Cancelled = 'cancelled',
  Completed = 'completed',
  Inprogress = 'inprogress'
}

export type GetTestListPartialQueryVariables = Exact<{ [key: string]: never; }>;


export type GetTestListPartialQuery = { __typename?: 'Query', listTests: Array<{ __typename?: 'Test', i: number, d: Date }> };

export type CreateAccountMutationVariables = Exact<{
  id: Scalars['UUID']['input'];
  remarks?: InputMaybe<Scalars['String']['input']>;
}>;


export type CreateAccountMutation = { __typename?: 'Mutation', createAccount: { __typename?: 'Account', id: string, remarks?: string | null, createdAt: Date } };

export type UpdateAccountMutationVariables = Exact<{
  id: Scalars['UUID']['input'];
  remarks?: InputMaybe<Scalars['String']['input']>;
}>;


export type UpdateAccountMutation = { __typename?: 'Mutation', updateAccount?: { __typename?: 'Account', id: string, remarks?: string | null, createdAt: Date } | null };

export type DeleteAccountMutationVariables = Exact<{
  id: Scalars['UUID']['input'];
}>;


export type DeleteAccountMutation = { __typename?: 'Mutation', deleteAccount: boolean };

export type CreateProfileMutationVariables = Exact<{
  idAccount: Scalars['UUID']['input'];
  name: Scalars['String']['input'];
  description?: InputMaybe<Scalars['String']['input']>;
}>;


export type CreateProfileMutation = { __typename?: 'Mutation', createProfile: { __typename?: 'Profile', id: number, name?: string | null, description?: string | null, variant?: ProfileType | null, owner: string, createdAt: Date, updatedAt?: Date | null, createdBy: string } };

export type UpdateProfileMutationVariables = Exact<{
  id: Scalars['Int']['input'];
  name: Scalars['String']['input'];
  description?: InputMaybe<Scalars['String']['input']>;
  variant?: InputMaybe<ProfileType>;
}>;


export type UpdateProfileMutation = { __typename?: 'Mutation', updateProfile?: { __typename?: 'Profile', id: number, name?: string | null, description?: string | null, variant?: ProfileType | null, owner: string, createdAt: Date, updatedAt?: Date | null, createdBy: string } | null };

export type DeleteProfileMutationVariables = Exact<{
  id: Scalars['Int']['input'];
}>;


export type DeleteProfileMutation = { __typename?: 'Mutation', deleteProfile: boolean };

export type CreateCommunityMutationVariables = Exact<{
  name: Scalars['String']['input'];
  description?: InputMaybe<Scalars['String']['input']>;
  variant: CommunityType;
  createdBy: Scalars['UUID']['input'];
}>;


export type CreateCommunityMutation = { __typename?: 'Mutation', createCommunity: { __typename?: 'Community', id: number, title: string, description?: string | null, variant: CommunityType, owner: string, createdAt: Date, updatedAt?: Date | null, createdBy: string } };

export type UpdateCommunityMutationVariables = Exact<{
  id: Scalars['Int']['input'];
  title: Scalars['String']['input'];
  description?: InputMaybe<Scalars['String']['input']>;
  variant: CommunityType;
}>;


export type UpdateCommunityMutation = { __typename?: 'Mutation', updateCommunity?: { __typename?: 'Community', id: number, title: string, description?: string | null, variant: CommunityType, owner: string, createdAt: Date, updatedAt?: Date | null, createdBy: string } | null };

export type DeleteCommunityMutationVariables = Exact<{
  id: Scalars['Int']['input'];
}>;


export type DeleteCommunityMutation = { __typename?: 'Mutation', deleteCommunity: boolean };

export type CreateCommitteeMutationVariables = Exact<{
  idProfile: Scalars['Int']['input'];
  idCommunity: Scalars['Int']['input'];
  memberRole: CommitteeRole;
}>;


export type CreateCommitteeMutation = { __typename?: 'Mutation', createCommittee: { __typename?: 'Committee', idProfile: number, idCommunity: number, memberRole: CommitteeRole, joined_at: Date } };

export type UpdateCommitteeMutationVariables = Exact<{
  idProfile: Scalars['Int']['input'];
  idCommunity: Scalars['Int']['input'];
  memberRole: CommitteeRole;
}>;


export type UpdateCommitteeMutation = { __typename?: 'Mutation', updateCommittee?: { __typename?: 'Committee', idProfile: number, idCommunity: number, memberRole: CommitteeRole, joined_at: Date } | null };

export type DeleteCommitteeMutationVariables = Exact<{
  idProfile: Scalars['Int']['input'];
  idCommunity: Scalars['Int']['input'];
}>;


export type DeleteCommitteeMutation = { __typename?: 'Mutation', deleteCommittee: boolean };

export type CreateCategoryMutationVariables = Exact<{
  name: Scalars['String']['input'];
  description: Scalars['String']['input'];
  parentId?: InputMaybe<Scalars['Int']['input']>;
}>;


export type CreateCategoryMutation = { __typename?: 'Mutation', createCategory: { __typename?: 'Category', id: number, title: string, description?: string | null, categoryParent?: number | null } };

export type UpdateCategoryMutationVariables = Exact<{
  id: Scalars['Int']['input'];
  name?: InputMaybe<Scalars['String']['input']>;
  description?: InputMaybe<Scalars['String']['input']>;
  parentId?: InputMaybe<Scalars['Int']['input']>;
}>;


export type UpdateCategoryMutation = { __typename?: 'Mutation', updateCategory: { __typename?: 'Category', id: number, title: string, description?: string | null, categoryParent?: number | null } };

export type DeleteCategoryMutationVariables = Exact<{
  id: Scalars['Int']['input'];
}>;


export type DeleteCategoryMutation = { __typename?: 'Mutation', deleteCategory: boolean };

export type CreateLocationMutationVariables = Exact<{
  addressLine1: Scalars['String']['input'];
  addressLine2?: InputMaybe<Scalars['String']['input']>;
  city: Scalars['String']['input'];
  state: Scalars['String']['input'];
  district?: InputMaybe<Scalars['String']['input']>;
  country: Scalars['String']['input'];
  coordinates?: InputMaybe<CoordinatesInput>;
  entranceNote?: InputMaybe<Scalars['String']['input']>;
}>;


export type CreateLocationMutation = { __typename?: 'Mutation', createLocation: { __typename?: 'Location', id: number, addressLine1: string, addressLine2?: string | null, city: string, state?: string | null, country: string, district?: string | null, entranceNote?: string | null, createdAt: Date, coordinates?: { __typename?: 'Coordinates', longitude: number, latitude: number } | null } };

export type UpdateLocationMutationVariables = Exact<{
  id: Scalars['Int']['input'];
  addressLine1?: InputMaybe<Scalars['String']['input']>;
  addressLine2?: InputMaybe<Scalars['String']['input']>;
  city?: InputMaybe<Scalars['String']['input']>;
  state?: InputMaybe<Scalars['String']['input']>;
  district?: InputMaybe<Scalars['String']['input']>;
  country?: InputMaybe<Scalars['String']['input']>;
  coordinates?: InputMaybe<CoordinatesInput>;
  entranceNote?: InputMaybe<Scalars['String']['input']>;
}>;


export type UpdateLocationMutation = { __typename?: 'Mutation', updateLocation: { __typename?: 'Location', id: number, addressLine1: string, addressLine2?: string | null, city: string, state?: string | null, country: string, district?: string | null, entranceNote?: string | null, createdAt: Date, coordinates?: { __typename?: 'Coordinates', longitude: number, latitude: number } | null } };

export type DeleteLocationMutationVariables = Exact<{
  id: Scalars['Int']['input'];
}>;


export type DeleteLocationMutation = { __typename?: 'Mutation', deleteLocation: boolean };

export type CreateItemMutationVariables = Exact<{
  variant: ItemType;
  intentAction: ItemIntentAction;
  title?: InputMaybe<Scalars['String']['input']>;
  description?: InputMaybe<Scalars['String']['input']>;
  category?: InputMaybe<Scalars['Int']['input']>;
  condition: ItemCondition;
  location?: InputMaybe<Scalars['Int']['input']>;
  createdBy?: InputMaybe<Scalars['UUID']['input']>;
  status?: InputMaybe<ItemStatus>;
}>;


export type CreateItemMutation = { __typename?: 'Mutation', createItem: { __typename?: 'Item', id: number, variant: ItemType, intentAction: ItemIntentAction, status?: ItemStatus | null, title?: string | null, description?: string | null, category?: number | null, condition?: ItemCondition | null, location?: number | null, viewsCount: number, isReported: boolean, createdAt: Date, updatedAt?: Date | null, createdBy?: string | null } };

export type UpdateItemMutationVariables = Exact<{
  id: Scalars['Int']['input'];
  title?: InputMaybe<Scalars['String']['input']>;
  description?: InputMaybe<Scalars['String']['input']>;
  category?: InputMaybe<Scalars['Int']['input']>;
  condition: ItemCondition;
  location?: InputMaybe<Scalars['Int']['input']>;
  status: ItemStatus;
}>;


export type UpdateItemMutation = { __typename?: 'Mutation', updateItem: { __typename?: 'Item', id: number, variant: ItemType, intentAction: ItemIntentAction, status?: ItemStatus | null, title?: string | null, description?: string | null, category?: number | null, condition?: ItemCondition | null, location?: number | null, viewsCount: number, isReported: boolean, createdAt: Date, updatedAt?: Date | null, createdBy?: string | null } };

export type DeleteItemMutationVariables = Exact<{
  id: Scalars['Int']['input'];
}>;


export type DeleteItemMutation = { __typename?: 'Mutation', deleteItem: boolean };

export type ReportItemMutationVariables = Exact<{
  id: Scalars['Int']['input'];
}>;


export type ReportItemMutation = { __typename?: 'Mutation', reportItem?: { __typename?: 'Item', id: number, variant: ItemType, intentAction: ItemIntentAction, status?: ItemStatus | null, title?: string | null, description?: string | null, category?: number | null, condition?: ItemCondition | null, location?: number | null, viewsCount: number, isReported: boolean, createdAt: Date, updatedAt?: Date | null, createdBy?: string | null } | null };

export type CreateCollectionMutationVariables = Exact<{
  idCommunity: Scalars['Int']['input'];
  title: Scalars['String']['input'];
  visibility: CollectionVisibility;
  variant: CollectionType;
  position: Scalars['Int']['input'];
}>;


export type CreateCollectionMutation = { __typename?: 'Mutation', createCollection: { __typename?: 'Collection', id: number, idCommunity?: number | null, title?: string | null, visibility: CollectionVisibility, variant?: CollectionType | null, position: number, createdAt: Date, updatedAt?: Date | null } };

export type UpdateCollectionMutationVariables = Exact<{
  id: Scalars['Int']['input'];
  title: Scalars['String']['input'];
  visibility: CollectionVisibility;
  variant: CollectionType;
  position: Scalars['Int']['input'];
}>;


export type UpdateCollectionMutation = { __typename?: 'Mutation', updateCollection: { __typename?: 'Collection', id: number, idCommunity?: number | null, title?: string | null, visibility: CollectionVisibility, variant?: CollectionType | null, position: number, createdAt: Date, updatedAt?: Date | null } };

export type DeleteCollectionMutationVariables = Exact<{
  id: Scalars['Int']['input'];
}>;


export type DeleteCollectionMutation = { __typename?: 'Mutation', deleteCollection: boolean };

export type CreateMediaMutationVariables = Exact<{
  idItem: Scalars['Int']['input'];
  url: Scalars['String']['input'];
  mediaType: MediaType;
  position: Scalars['Int']['input'];
}>;


export type CreateMediaMutation = { __typename?: 'Mutation', createMedia: { __typename?: 'Media', id: number, idItem: number, caption?: string | null, url: string, variant: MediaType, createdAt: Date } };

export type UpdateMediaMutationVariables = Exact<{
  id: Scalars['Int']['input'];
  url?: InputMaybe<Scalars['String']['input']>;
  mediaType?: InputMaybe<MediaType>;
  position?: InputMaybe<Scalars['Int']['input']>;
}>;


export type UpdateMediaMutation = { __typename?: 'Mutation', updateMedia: { __typename?: 'Media', id: number, idItem: number, caption?: string | null, url: string, variant: MediaType, createdAt: Date } };

export type DeleteMediaMutationVariables = Exact<{
  id: Scalars['Int']['input'];
}>;


export type DeleteMediaMutation = { __typename?: 'Mutation', deleteMedia: boolean };

export type CreatePublishMutationVariables = Exact<{
  idItem: Scalars['Int']['input'];
  idCollection: Scalars['Int']['input'];
  note?: InputMaybe<Scalars['String']['input']>;
  position: Scalars['Int']['input'];
  createdBy: Scalars['UUID']['input'];
}>;


export type CreatePublishMutation = { __typename?: 'Mutation', createPublish: { __typename?: 'Publish', idItem: number, idCollection: number, note?: string | null, position: number, addedBy?: string | null, posted_on: Date } };

export type UpdatePublishMutationVariables = Exact<{
  idItem: Scalars['Int']['input'];
  idCollection: Scalars['Int']['input'];
  note?: InputMaybe<Scalars['String']['input']>;
  position: Scalars['Int']['input'];
}>;


export type UpdatePublishMutation = { __typename?: 'Mutation', updatePublish: { __typename?: 'Publish', idItem: number, idCollection: number, note?: string | null, position: number, addedBy?: string | null, posted_on: Date } };

export type DeletePublishMutationVariables = Exact<{
  idItem: Scalars['Int']['input'];
  idCollection: Scalars['Int']['input'];
}>;


export type DeletePublishMutation = { __typename?: 'Mutation', deletePublish: boolean };

export type CreateMessageMutationVariables = Exact<{
  idTransaction: Scalars['Int']['input'];
  idSender: Scalars['UUID']['input'];
  variant: MessageType;
  content: Scalars['String']['input'];
}>;


export type CreateMessageMutation = { __typename?: 'Mutation', create: { __typename?: 'Message', id: number, idSender?: number | null, idTransaction: number, variant?: MessageType | null, content: string, sent_at: Date, updatedAt?: Date | null } };

export type UpdateTransactionMutationVariables = Exact<{
  id: Scalars['Int']['input'];
  status: TransactionStatus;
}>;


export type UpdateTransactionMutation = { __typename?: 'Mutation', update: { __typename?: 'Transaction', id: number, idPledge: number, idSchedule?: number | null, idLocation?: number | null, status: TransactionStatus, createdAt: Date, updatedAt?: Date | null } };

export type ListAccountsQueryVariables = Exact<{ [key: string]: never; }>;


export type ListAccountsQuery = { __typename?: 'Query', listAccounts: Array<{ __typename?: 'Account', id: string, remarks?: string | null, createdAt: Date }> };

export type FindAccountQueryVariables = Exact<{
  id: Scalars['UUID']['input'];
}>;


export type FindAccountQuery = { __typename?: 'Query', findAccount?: { __typename?: 'Account', id: string, remarks?: string | null, createdAt: Date } | null };

export type ListProfilesQueryVariables = Exact<{ [key: string]: never; }>;


export type ListProfilesQuery = { __typename?: 'Query', listProfiles: Array<{ __typename?: 'Profile', id: number, name?: string | null, description?: string | null, variant?: ProfileType | null, owner: string, createdAt: Date, updatedAt?: Date | null, createdBy: string }> };

export type FindProfileQueryVariables = Exact<{
  id: Scalars['Int']['input'];
}>;


export type FindProfileQuery = { __typename?: 'Query', findProfile?: { __typename?: 'Profile', id: number, name?: string | null, description?: string | null, variant?: ProfileType | null, owner: string, createdAt: Date, updatedAt?: Date | null, createdBy: string } | null };

export type ListCommunitiesQueryVariables = Exact<{ [key: string]: never; }>;


export type ListCommunitiesQuery = { __typename?: 'Query', listCommunities: Array<{ __typename?: 'Community', id: number, title: string, description?: string | null, variant: CommunityType, owner: string, createdAt: Date, updatedAt?: Date | null, createdBy: string }> };

export type FindCommunityQueryVariables = Exact<{
  id: Scalars['Int']['input'];
}>;


export type FindCommunityQuery = { __typename?: 'Query', findCommunity?: { __typename?: 'Community', id: number, title: string, description?: string | null, variant: CommunityType, owner: string, createdAt: Date, updatedAt?: Date | null, createdBy: string } | null };

export type ListCommitteesQueryVariables = Exact<{ [key: string]: never; }>;


export type ListCommitteesQuery = { __typename?: 'Query', listCommittees: Array<{ __typename?: 'Committee', idProfile: number, idCommunity: number, memberRole: CommitteeRole, joined_at: Date }> };

export type FindCommitteeQueryVariables = Exact<{
  id: Scalars['UUID']['input'];
}>;


export type FindCommitteeQuery = { __typename?: 'Query', findCommittee?: { __typename?: 'Committee', idProfile: number, idCommunity: number, memberRole: CommitteeRole, joined_at: Date } | null };

export type ListCategoriesQueryVariables = Exact<{ [key: string]: never; }>;


export type ListCategoriesQuery = { __typename?: 'Query', listCategories: Array<{ __typename?: 'Category', id: number, title: string, description?: string | null, categoryParent?: number | null }> };

export type FindCategoryQueryVariables = Exact<{
  id: Scalars['Int']['input'];
}>;


export type FindCategoryQuery = { __typename?: 'Query', findCategory?: { __typename?: 'Category', id: number, title: string, description?: string | null, categoryParent?: number | null } | null };

export type ListItemsQueryVariables = Exact<{ [key: string]: never; }>;


export type ListItemsQuery = { __typename?: 'Query', listItems: Array<{ __typename?: 'Item', id: number, variant: ItemType, intentAction: ItemIntentAction, status?: ItemStatus | null, title?: string | null, description?: string | null, category?: number | null, condition?: ItemCondition | null, location?: number | null, viewsCount: number, isReported: boolean, createdAt: Date, updatedAt?: Date | null, createdBy?: string | null }> };

export type FindItemQueryVariables = Exact<{
  id: Scalars['Int']['input'];
}>;


export type FindItemQuery = { __typename?: 'Query', findItem?: { __typename?: 'Item', id: number, variant: ItemType, intentAction: ItemIntentAction, status?: ItemStatus | null, title?: string | null, description?: string | null, category?: number | null, condition?: ItemCondition | null, location?: number | null, viewsCount: number, isReported: boolean, createdAt: Date, updatedAt?: Date | null, createdBy?: string | null } | null };

export type ListLocationsQueryVariables = Exact<{ [key: string]: never; }>;


export type ListLocationsQuery = { __typename?: 'Query', listLocations: Array<{ __typename?: 'Location', id: number, addressLine1: string, addressLine2?: string | null, city: string, state?: string | null, country: string, district?: string | null, entranceNote?: string | null, createdAt: Date, coordinates?: { __typename?: 'Coordinates', longitude: number, latitude: number } | null }> };

export type FindLocationQueryVariables = Exact<{
  id: Scalars['Int']['input'];
}>;


export type FindLocationQuery = { __typename?: 'Query', findLocation?: { __typename?: 'Location', id: number, addressLine1: string, addressLine2?: string | null, city: string, state?: string | null, country: string, district?: string | null, entranceNote?: string | null, createdAt: Date, coordinates?: { __typename?: 'Coordinates', longitude: number, latitude: number } | null } | null };

export type ListCollectionsQueryVariables = Exact<{ [key: string]: never; }>;


export type ListCollectionsQuery = { __typename?: 'Query', listCollections: Array<{ __typename?: 'Collection', id: number, idCommunity?: number | null, title?: string | null, visibility: CollectionVisibility, variant?: CollectionType | null, position: number, createdAt: Date, updatedAt?: Date | null }> };

export type FindCollectionQueryVariables = Exact<{
  id: Scalars['Int']['input'];
}>;


export type FindCollectionQuery = { __typename?: 'Query', findCollection?: { __typename?: 'Collection', id: number, idCommunity?: number | null, title?: string | null, visibility: CollectionVisibility, variant?: CollectionType | null, position: number, createdAt: Date, updatedAt?: Date | null } | null };

export type ListMediaQueryVariables = Exact<{ [key: string]: never; }>;


export type ListMediaQuery = { __typename?: 'Query', listMedia: Array<{ __typename?: 'Media', id: number, idItem: number, caption?: string | null, url: string, variant: MediaType, createdAt: Date }> };

export type FindMediaQueryVariables = Exact<{
  id: Scalars['Int']['input'];
}>;


export type FindMediaQuery = { __typename?: 'Query', findMedia?: { __typename?: 'Media', id: number, idItem: number, caption?: string | null, url: string, variant: MediaType, createdAt: Date } | null };

export type ListPublishesQueryVariables = Exact<{ [key: string]: never; }>;


export type ListPublishesQuery = { __typename?: 'Query', listPublishes: Array<{ __typename?: 'Publish', idItem: number, idCollection: number, note?: string | null, position: number, addedBy?: string | null, posted_on: Date }> };

export type FindPublishQueryVariables = Exact<{
  id: Scalars['Int']['input'];
}>;


export type FindPublishQuery = { __typename?: 'Query', findPublish?: { __typename?: 'Publish', idItem: number, idCollection: number, note?: string | null, position: number, addedBy?: string | null, posted_on: Date } | null };

export type ListMessagesQueryVariables = Exact<{ [key: string]: never; }>;


export type ListMessagesQuery = { __typename?: 'Query', listMessages: Array<{ __typename?: 'Message', id: number, idSender?: number | null, idTransaction: number, variant?: MessageType | null, content: string, sent_at: Date, updatedAt?: Date | null }> };

export type FindMessageQueryVariables = Exact<{
  id: Scalars['Int']['input'];
}>;


export type FindMessageQuery = { __typename?: 'Query', findMessage?: { __typename?: 'Message', id: number, idSender?: number | null, idTransaction: number, variant?: MessageType | null, content: string, sent_at: Date, updatedAt?: Date | null } | null };

export type GetAccountListQueryVariables = Exact<{ [key: string]: never; }>;


export type GetAccountListQuery = { __typename?: 'Query', listAccounts: Array<{ __typename?: 'Account', id: string, createdAt: Date }> };

export type GetTestListQueryVariables = Exact<{ [key: string]: never; }>;


export type GetTestListQuery = { __typename?: 'Query', listTests: Array<{ __typename?: 'Test', i: number, s: string, d: Date }> };

export class TypedDocumentString<TResult, TVariables>
  extends String
  implements DocumentTypeDecoration<TResult, TVariables>
{
  __apiType?: DocumentTypeDecoration<TResult, TVariables>['__apiType'];
  private value: string;
  public __meta__?: Record<string, any> | undefined;

  constructor(value: string, __meta__?: Record<string, any> | undefined) {
    super(value);
    this.value = value;
    this.__meta__ = __meta__;
  }

  toString(): string & DocumentTypeDecoration<TResult, TVariables> {
    return this.value;
  }
}

export const GetTestListPartialDocument = new TypedDocumentString(`
    query GetTestListPartial {
  listTests {
    i
    d
  }
}
    `) as unknown as TypedDocumentString<GetTestListPartialQuery, GetTestListPartialQueryVariables>;
export const CreateAccountDocument = new TypedDocumentString(`
    mutation CreateAccount($id: UUID!, $remarks: String) {
  createAccount(id: $id, remarks: $remarks) {
    id
    remarks
    createdAt
  }
}
    `) as unknown as TypedDocumentString<CreateAccountMutation, CreateAccountMutationVariables>;
export const UpdateAccountDocument = new TypedDocumentString(`
    mutation UpdateAccount($id: UUID!, $remarks: String) {
  updateAccount(id: $id, remarks: $remarks) {
    id
    remarks
    createdAt
  }
}
    `) as unknown as TypedDocumentString<UpdateAccountMutation, UpdateAccountMutationVariables>;
export const DeleteAccountDocument = new TypedDocumentString(`
    mutation DeleteAccount($id: UUID!) {
  deleteAccount(id: $id)
}
    `) as unknown as TypedDocumentString<DeleteAccountMutation, DeleteAccountMutationVariables>;
export const CreateProfileDocument = new TypedDocumentString(`
    mutation CreateProfile($idAccount: UUID!, $name: String!, $description: String) {
  createProfile(idAccount: $idAccount, name: $name, description: $description) {
    id
    name
    description
    variant
    owner
    createdAt
    updatedAt
    createdBy
  }
}
    `) as unknown as TypedDocumentString<CreateProfileMutation, CreateProfileMutationVariables>;
export const UpdateProfileDocument = new TypedDocumentString(`
    mutation UpdateProfile($id: Int!, $name: String!, $description: String, $variant: ProfileType) {
  updateProfile(
    id: $id
    name: $name
    description: $description
    variant: $variant
  ) {
    id
    name
    description
    variant
    owner
    createdAt
    updatedAt
    createdBy
  }
}
    `) as unknown as TypedDocumentString<UpdateProfileMutation, UpdateProfileMutationVariables>;
export const DeleteProfileDocument = new TypedDocumentString(`
    mutation DeleteProfile($id: Int!) {
  deleteProfile(id: $id)
}
    `) as unknown as TypedDocumentString<DeleteProfileMutation, DeleteProfileMutationVariables>;
export const CreateCommunityDocument = new TypedDocumentString(`
    mutation CreateCommunity($name: String!, $description: String, $variant: CommunityType!, $createdBy: UUID!) {
  createCommunity(
    name: $name
    description: $description
    variant: $variant
    createdBy: $createdBy
  ) {
    id
    title
    description
    variant
    owner
    createdAt
    updatedAt
    createdBy
  }
}
    `) as unknown as TypedDocumentString<CreateCommunityMutation, CreateCommunityMutationVariables>;
export const UpdateCommunityDocument = new TypedDocumentString(`
    mutation UpdateCommunity($id: Int!, $title: String!, $description: String, $variant: CommunityType!) {
  updateCommunity(
    id: $id
    title: $title
    description: $description
    variant: $variant
  ) {
    id
    title
    description
    variant
    owner
    createdAt
    updatedAt
    createdBy
  }
}
    `) as unknown as TypedDocumentString<UpdateCommunityMutation, UpdateCommunityMutationVariables>;
export const DeleteCommunityDocument = new TypedDocumentString(`
    mutation DeleteCommunity($id: Int!) {
  deleteCommunity(id: $id)
}
    `) as unknown as TypedDocumentString<DeleteCommunityMutation, DeleteCommunityMutationVariables>;
export const CreateCommitteeDocument = new TypedDocumentString(`
    mutation CreateCommittee($idProfile: Int!, $idCommunity: Int!, $memberRole: CommitteeRole!) {
  createCommittee(
    idProfile: $idProfile
    idCommunity: $idCommunity
    memberRole: $memberRole
  ) {
    idProfile
    idCommunity
    memberRole
    joined_at
  }
}
    `) as unknown as TypedDocumentString<CreateCommitteeMutation, CreateCommitteeMutationVariables>;
export const UpdateCommitteeDocument = new TypedDocumentString(`
    mutation UpdateCommittee($idProfile: Int!, $idCommunity: Int!, $memberRole: CommitteeRole!) {
  updateCommittee(
    idProfile: $idProfile
    idCommunity: $idCommunity
    memberRole: $memberRole
  ) {
    idProfile
    idCommunity
    memberRole
    joined_at
  }
}
    `) as unknown as TypedDocumentString<UpdateCommitteeMutation, UpdateCommitteeMutationVariables>;
export const DeleteCommitteeDocument = new TypedDocumentString(`
    mutation DeleteCommittee($idProfile: Int!, $idCommunity: Int!) {
  deleteCommittee(idProfile: $idProfile, idCommunity: $idCommunity)
}
    `) as unknown as TypedDocumentString<DeleteCommitteeMutation, DeleteCommitteeMutationVariables>;
export const CreateCategoryDocument = new TypedDocumentString(`
    mutation CreateCategory($name: String!, $description: String!, $parentId: Int) {
  createCategory(name: $name, description: $description, parentId: $parentId) {
    id
    title
    description
    categoryParent
  }
}
    `) as unknown as TypedDocumentString<CreateCategoryMutation, CreateCategoryMutationVariables>;
export const UpdateCategoryDocument = new TypedDocumentString(`
    mutation UpdateCategory($id: Int!, $name: String, $description: String, $parentId: Int) {
  updateCategory(
    id: $id
    name: $name
    description: $description
    parentId: $parentId
  ) {
    id
    title
    description
    categoryParent
  }
}
    `) as unknown as TypedDocumentString<UpdateCategoryMutation, UpdateCategoryMutationVariables>;
export const DeleteCategoryDocument = new TypedDocumentString(`
    mutation DeleteCategory($id: Int!) {
  deleteCategory(id: $id)
}
    `) as unknown as TypedDocumentString<DeleteCategoryMutation, DeleteCategoryMutationVariables>;
export const CreateLocationDocument = new TypedDocumentString(`
    mutation CreateLocation($addressLine1: String!, $addressLine2: String, $city: String!, $state: String!, $district: String, $country: String!, $coordinates: CoordinatesInput, $entranceNote: String) {
  createLocation(
    addressLine1: $addressLine1
    addressLine2: $addressLine2
    city: $city
    state: $state
    district: $district
    country: $country
    coordinates: $coordinates
    entranceNote: $entranceNote
  ) {
    id
    addressLine1
    addressLine2
    city
    state
    country
    district
    entranceNote
    createdAt
    coordinates {
      longitude
      latitude
    }
  }
}
    `) as unknown as TypedDocumentString<CreateLocationMutation, CreateLocationMutationVariables>;
export const UpdateLocationDocument = new TypedDocumentString(`
    mutation UpdateLocation($id: Int!, $addressLine1: String, $addressLine2: String, $city: String, $state: String, $district: String, $country: String, $coordinates: CoordinatesInput, $entranceNote: String) {
  updateLocation(
    id: $id
    addressLine1: $addressLine1
    addressLine2: $addressLine2
    city: $city
    state: $state
    district: $district
    country: $country
    coordinates: $coordinates
    entranceNote: $entranceNote
  ) {
    id
    addressLine1
    addressLine2
    city
    state
    country
    district
    entranceNote
    createdAt
    coordinates {
      longitude
      latitude
    }
  }
}
    `) as unknown as TypedDocumentString<UpdateLocationMutation, UpdateLocationMutationVariables>;
export const DeleteLocationDocument = new TypedDocumentString(`
    mutation DeleteLocation($id: Int!) {
  deleteLocation(id: $id)
}
    `) as unknown as TypedDocumentString<DeleteLocationMutation, DeleteLocationMutationVariables>;
export const CreateItemDocument = new TypedDocumentString(`
    mutation CreateItem($variant: ItemType!, $intentAction: ItemIntentAction!, $title: String, $description: String, $category: Int, $condition: ItemCondition!, $location: Int, $createdBy: UUID, $status: ItemStatus) {
  createItem(
    variant: $variant
    intentAction: $intentAction
    title: $title
    description: $description
    category: $category
    condition: $condition
    location: $location
    createdBy: $createdBy
    status: $status
  ) {
    id
    variant
    intentAction
    status
    title
    description
    category
    condition
    location
    viewsCount
    isReported
    createdAt
    updatedAt
    createdBy
  }
}
    `) as unknown as TypedDocumentString<CreateItemMutation, CreateItemMutationVariables>;
export const UpdateItemDocument = new TypedDocumentString(`
    mutation UpdateItem($id: Int!, $title: String, $description: String, $category: Int, $condition: ItemCondition!, $location: Int, $status: ItemStatus!) {
  updateItem(
    id: $id
    title: $title
    description: $description
    category: $category
    condition: $condition
    location: $location
    status: $status
  ) {
    id
    variant
    intentAction
    status
    title
    description
    category
    condition
    location
    viewsCount
    isReported
    createdAt
    updatedAt
    createdBy
  }
}
    `) as unknown as TypedDocumentString<UpdateItemMutation, UpdateItemMutationVariables>;
export const DeleteItemDocument = new TypedDocumentString(`
    mutation DeleteItem($id: Int!) {
  deleteItem(id: $id)
}
    `) as unknown as TypedDocumentString<DeleteItemMutation, DeleteItemMutationVariables>;
export const ReportItemDocument = new TypedDocumentString(`
    mutation ReportItem($id: Int!) {
  reportItem(id: $id) {
    id
    variant
    intentAction
    status
    title
    description
    category
    condition
    location
    viewsCount
    isReported
    createdAt
    updatedAt
    createdBy
  }
}
    `) as unknown as TypedDocumentString<ReportItemMutation, ReportItemMutationVariables>;
export const CreateCollectionDocument = new TypedDocumentString(`
    mutation CreateCollection($idCommunity: Int!, $title: String!, $visibility: CollectionVisibility!, $variant: CollectionType!, $position: Int!) {
  createCollection(
    idCommunity: $idCommunity
    title: $title
    visibility: $visibility
    variant: $variant
    position: $position
  ) {
    id
    idCommunity
    title
    visibility
    variant
    position
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<CreateCollectionMutation, CreateCollectionMutationVariables>;
export const UpdateCollectionDocument = new TypedDocumentString(`
    mutation UpdateCollection($id: Int!, $title: String!, $visibility: CollectionVisibility!, $variant: CollectionType!, $position: Int!) {
  updateCollection(
    id: $id
    title: $title
    visibility: $visibility
    variant: $variant
    position: $position
  ) {
    id
    idCommunity
    title
    visibility
    variant
    position
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<UpdateCollectionMutation, UpdateCollectionMutationVariables>;
export const DeleteCollectionDocument = new TypedDocumentString(`
    mutation DeleteCollection($id: Int!) {
  deleteCollection(id: $id)
}
    `) as unknown as TypedDocumentString<DeleteCollectionMutation, DeleteCollectionMutationVariables>;
export const CreateMediaDocument = new TypedDocumentString(`
    mutation CreateMedia($idItem: Int!, $url: String!, $mediaType: MediaType!, $position: Int!) {
  createMedia(
    idItem: $idItem
    url: $url
    mediaType: $mediaType
    position: $position
  ) {
    id
    idItem
    caption
    url
    variant
    createdAt
  }
}
    `) as unknown as TypedDocumentString<CreateMediaMutation, CreateMediaMutationVariables>;
export const UpdateMediaDocument = new TypedDocumentString(`
    mutation UpdateMedia($id: Int!, $url: String, $mediaType: MediaType, $position: Int) {
  updateMedia(id: $id, url: $url, mediaType: $mediaType, position: $position) {
    id
    idItem
    caption
    url
    variant
    createdAt
  }
}
    `) as unknown as TypedDocumentString<UpdateMediaMutation, UpdateMediaMutationVariables>;
export const DeleteMediaDocument = new TypedDocumentString(`
    mutation DeleteMedia($id: Int!) {
  deleteMedia(id: $id)
}
    `) as unknown as TypedDocumentString<DeleteMediaMutation, DeleteMediaMutationVariables>;
export const CreatePublishDocument = new TypedDocumentString(`
    mutation CreatePublish($idItem: Int!, $idCollection: Int!, $note: String, $position: Int!, $createdBy: UUID!) {
  createPublish(
    idItem: $idItem
    idCollection: $idCollection
    note: $note
    position: $position
    createdBy: $createdBy
  ) {
    idItem
    idCollection
    note
    position
    addedBy
    posted_on
  }
}
    `) as unknown as TypedDocumentString<CreatePublishMutation, CreatePublishMutationVariables>;
export const UpdatePublishDocument = new TypedDocumentString(`
    mutation UpdatePublish($idItem: Int!, $idCollection: Int!, $note: String, $position: Int!) {
  updatePublish(
    idItem: $idItem
    idCollection: $idCollection
    note: $note
    position: $position
  ) {
    idItem
    idCollection
    note
    position
    addedBy
    posted_on
  }
}
    `) as unknown as TypedDocumentString<UpdatePublishMutation, UpdatePublishMutationVariables>;
export const DeletePublishDocument = new TypedDocumentString(`
    mutation DeletePublish($idItem: Int!, $idCollection: Int!) {
  deletePublish(idItem: $idItem, idCollection: $idCollection)
}
    `) as unknown as TypedDocumentString<DeletePublishMutation, DeletePublishMutationVariables>;
export const CreateMessageDocument = new TypedDocumentString(`
    mutation CreateMessage($idTransaction: Int!, $idSender: UUID!, $variant: MessageType!, $content: String!) {
  create(
    idTransaction: $idTransaction
    idSender: $idSender
    variant: $variant
    content: $content
  ) {
    id
    idSender
    idTransaction
    variant
    content
    sent_at
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<CreateMessageMutation, CreateMessageMutationVariables>;
export const UpdateTransactionDocument = new TypedDocumentString(`
    mutation UpdateTransaction($id: Int!, $status: TransactionStatus!) {
  update(id: $id, status: $status) {
    id
    idPledge
    idSchedule
    idLocation
    status
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<UpdateTransactionMutation, UpdateTransactionMutationVariables>;
export const ListAccountsDocument = new TypedDocumentString(`
    query ListAccounts {
  listAccounts {
    id
    remarks
    createdAt
  }
}
    `) as unknown as TypedDocumentString<ListAccountsQuery, ListAccountsQueryVariables>;
export const FindAccountDocument = new TypedDocumentString(`
    query FindAccount($id: UUID!) {
  findAccount(id: $id) {
    id
    remarks
    createdAt
  }
}
    `) as unknown as TypedDocumentString<FindAccountQuery, FindAccountQueryVariables>;
export const ListProfilesDocument = new TypedDocumentString(`
    query ListProfiles {
  listProfiles {
    id
    name
    description
    variant
    owner
    createdAt
    updatedAt
    createdBy
  }
}
    `) as unknown as TypedDocumentString<ListProfilesQuery, ListProfilesQueryVariables>;
export const FindProfileDocument = new TypedDocumentString(`
    query FindProfile($id: Int!) {
  findProfile(id: $id) {
    id
    name
    description
    variant
    owner
    createdAt
    updatedAt
    createdBy
  }
}
    `) as unknown as TypedDocumentString<FindProfileQuery, FindProfileQueryVariables>;
export const ListCommunitiesDocument = new TypedDocumentString(`
    query ListCommunities {
  listCommunities {
    id
    title
    description
    variant
    owner
    createdAt
    updatedAt
    createdBy
  }
}
    `) as unknown as TypedDocumentString<ListCommunitiesQuery, ListCommunitiesQueryVariables>;
export const FindCommunityDocument = new TypedDocumentString(`
    query FindCommunity($id: Int!) {
  findCommunity(id: $id) {
    id
    title
    description
    variant
    owner
    createdAt
    updatedAt
    createdBy
  }
}
    `) as unknown as TypedDocumentString<FindCommunityQuery, FindCommunityQueryVariables>;
export const ListCommitteesDocument = new TypedDocumentString(`
    query ListCommittees {
  listCommittees {
    idProfile
    idCommunity
    memberRole
    joined_at
  }
}
    `) as unknown as TypedDocumentString<ListCommitteesQuery, ListCommitteesQueryVariables>;
export const FindCommitteeDocument = new TypedDocumentString(`
    query FindCommittee($id: UUID!) {
  findCommittee(id: $id) {
    idProfile
    idCommunity
    memberRole
    joined_at
  }
}
    `) as unknown as TypedDocumentString<FindCommitteeQuery, FindCommitteeQueryVariables>;
export const ListCategoriesDocument = new TypedDocumentString(`
    query ListCategories {
  listCategories {
    id
    title
    description
    categoryParent
  }
}
    `) as unknown as TypedDocumentString<ListCategoriesQuery, ListCategoriesQueryVariables>;
export const FindCategoryDocument = new TypedDocumentString(`
    query FindCategory($id: Int!) {
  findCategory(id: $id) {
    id
    title
    description
    categoryParent
  }
}
    `) as unknown as TypedDocumentString<FindCategoryQuery, FindCategoryQueryVariables>;
export const ListItemsDocument = new TypedDocumentString(`
    query ListItems {
  listItems {
    id
    variant
    intentAction
    status
    title
    description
    category
    condition
    location
    viewsCount
    isReported
    createdAt
    updatedAt
    createdBy
  }
}
    `) as unknown as TypedDocumentString<ListItemsQuery, ListItemsQueryVariables>;
export const FindItemDocument = new TypedDocumentString(`
    query FindItem($id: Int!) {
  findItem(id: $id) {
    id
    variant
    intentAction
    status
    title
    description
    category
    condition
    location
    viewsCount
    isReported
    createdAt
    updatedAt
    createdBy
  }
}
    `) as unknown as TypedDocumentString<FindItemQuery, FindItemQueryVariables>;
export const ListLocationsDocument = new TypedDocumentString(`
    query ListLocations {
  listLocations {
    id
    addressLine1
    addressLine2
    city
    state
    country
    district
    entranceNote
    createdAt
    coordinates {
      longitude
      latitude
    }
  }
}
    `) as unknown as TypedDocumentString<ListLocationsQuery, ListLocationsQueryVariables>;
export const FindLocationDocument = new TypedDocumentString(`
    query FindLocation($id: Int!) {
  findLocation(id: $id) {
    id
    addressLine1
    addressLine2
    city
    state
    country
    district
    entranceNote
    createdAt
    coordinates {
      longitude
      latitude
    }
  }
}
    `) as unknown as TypedDocumentString<FindLocationQuery, FindLocationQueryVariables>;
export const ListCollectionsDocument = new TypedDocumentString(`
    query ListCollections {
  listCollections {
    id
    idCommunity
    title
    visibility
    variant
    position
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<ListCollectionsQuery, ListCollectionsQueryVariables>;
export const FindCollectionDocument = new TypedDocumentString(`
    query FindCollection($id: Int!) {
  findCollection(id: $id) {
    id
    idCommunity
    title
    visibility
    variant
    position
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<FindCollectionQuery, FindCollectionQueryVariables>;
export const ListMediaDocument = new TypedDocumentString(`
    query ListMedia {
  listMedia {
    id
    idItem
    caption
    url
    variant
    createdAt
  }
}
    `) as unknown as TypedDocumentString<ListMediaQuery, ListMediaQueryVariables>;
export const FindMediaDocument = new TypedDocumentString(`
    query FindMedia($id: Int!) {
  findMedia(id: $id) {
    id
    idItem
    caption
    url
    variant
    createdAt
  }
}
    `) as unknown as TypedDocumentString<FindMediaQuery, FindMediaQueryVariables>;
export const ListPublishesDocument = new TypedDocumentString(`
    query ListPublishes {
  listPublishes {
    idItem
    idCollection
    note
    position
    addedBy
    posted_on
  }
}
    `) as unknown as TypedDocumentString<ListPublishesQuery, ListPublishesQueryVariables>;
export const FindPublishDocument = new TypedDocumentString(`
    query FindPublish($id: Int!) {
  findPublish(id: $id) {
    idItem
    idCollection
    note
    position
    addedBy
    posted_on
  }
}
    `) as unknown as TypedDocumentString<FindPublishQuery, FindPublishQueryVariables>;
export const ListMessagesDocument = new TypedDocumentString(`
    query ListMessages {
  listMessages {
    id
    idSender
    idTransaction
    variant
    content
    sent_at
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<ListMessagesQuery, ListMessagesQueryVariables>;
export const FindMessageDocument = new TypedDocumentString(`
    query FindMessage($id: Int!) {
  findMessage(id: $id) {
    id
    idSender
    idTransaction
    variant
    content
    sent_at
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<FindMessageQuery, FindMessageQueryVariables>;
export const GetAccountListDocument = new TypedDocumentString(`
    query GetAccountList {
  listAccounts {
    id
    createdAt
  }
}
    `) as unknown as TypedDocumentString<GetAccountListQuery, GetAccountListQueryVariables>;
export const GetTestListDocument = new TypedDocumentString(`
    query GetTestList {
  listTests {
    i
    s
    d
  }
}
    `) as unknown as TypedDocumentString<GetTestListQuery, GetTestListQueryVariables>;