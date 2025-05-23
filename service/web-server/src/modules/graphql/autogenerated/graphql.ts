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
   * * [RFC4122: A Universally Unique IDentifier (UUID) URN Namespace](http://tools.ietf.org/html/rfc4122)
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
  createdAt: Scalars['DateTime']['output'];
  description?: Maybe<Scalars['String']['output']>;
  id: Scalars['Int']['output'];
  title: Scalars['String']['output'];
  updatedAt: Scalars['DateTime']['output'];
};

export type Collection = {
  __typename?: 'Collection';
  createdAt: Scalars['DateTime']['output'];
  id: Scalars['Int']['output'];
  idCommunity: Scalars['Int']['output'];
  position: Scalars['Int']['output'];
  title: Scalars['String']['output'];
  type: CollectionType;
  updatedAt: Scalars['DateTime']['output'];
  visibility: CollectionVisibility;
};

export enum CollectionType {
  Featured = 'FEATURED',
  Regular = 'REGULAR'
}

export enum CollectionVisibility {
  Public = 'PUBLIC',
  Restricted = 'RESTRICTED'
}

export type Committee = {
  __typename?: 'Committee';
  idCommunity: Scalars['Int']['output'];
  idProfile: Scalars['Int']['output'];
  joinedAt: Scalars['DateTime']['output'];
  memberRole: CommitteeRole;
};

export enum CommitteeRole {
  Member = 'MEMBER',
  Organizer = 'ORGANIZER'
}

export type Community = {
  __typename?: 'Community';
  createdAt: Scalars['DateTime']['output'];
  createdBy: Scalars['UUID']['output'];
  description?: Maybe<Scalars['String']['output']>;
  id: Scalars['Int']['output'];
  owner: Scalars['UUID']['output'];
  title?: Maybe<Scalars['String']['output']>;
  type: CommunityType;
  updatedAt?: Maybe<Scalars['DateTime']['output']>;
};

export enum CommunityType {
  Organized = 'ORGANIZED',
  Solo = 'SOLO'
}

export type Item = {
  __typename?: 'Item';
  condition: ItemCondition;
  createdAt: Scalars['DateTime']['output'];
  description?: Maybe<Scalars['String']['output']>;
  id: Scalars['Int']['output'];
  idCategory: Scalars['Int']['output'];
  idLocation: Scalars['Int']['output'];
  intentAction: ItemIntentAction;
  quantity: Scalars['Int']['output'];
  status: ItemStatus;
  title: Scalars['String']['output'];
  type: ItemType;
  updatedAt: Scalars['DateTime']['output'];
};

export enum ItemCondition {
  BrandNew = 'BRAND_NEW',
  PreOwnedBarelyUsed = 'PRE_OWNED_BARELY_USED',
  PreOwnedDamaged = 'PRE_OWNED_DAMAGED',
  PreOwnedUsable = 'PRE_OWNED_USABLE'
}

export enum ItemIntentAction {
  Offer = 'OFFER',
  Request = 'REQUEST'
}

export enum ItemStatus {
  Active = 'ACTIVE',
  Archived = 'ARCHIVED',
  Disabled = 'DISABLED',
  Draft = 'DRAFT'
}

export enum ItemType {
  Inquiry = 'INQUIRY',
  InKind = 'IN_KIND',
  Monetary = 'MONETARY',
  Service = 'SERVICE'
}

export type Location = {
  __typename?: 'Location';
  addressLine1: Scalars['String']['output'];
  addressLine2?: Maybe<Scalars['String']['output']>;
  city: Scalars['String']['output'];
  country: Scalars['String']['output'];
  createdAt: Scalars['DateTime']['output'];
  district?: Maybe<Scalars['String']['output']>;
  entranceNote?: Maybe<Scalars['String']['output']>;
  geom?: Maybe<Scalars['String']['output']>;
  id: Scalars['Int']['output'];
  state: Scalars['String']['output'];
  updatedAt: Scalars['DateTime']['output'];
};

export type Media = {
  __typename?: 'Media';
  caption?: Maybe<Scalars['String']['output']>;
  createdAt: Scalars['DateTime']['output'];
  id: Scalars['Int']['output'];
  idItem: Scalars['Int']['output'];
  type: MediaType;
  updatedAt: Scalars['DateTime']['output'];
  url: Scalars['String']['output'];
};

export enum MediaType {
  Image = 'IMAGE',
  Video = 'VIDEO'
}

export type Message = {
  __typename?: 'Message';
  content: Scalars['String']['output'];
  createdAt: Scalars['DateTime']['output'];
  id: Scalars['Int']['output'];
  idSender: Scalars['UUID']['output'];
  idTransaction: Scalars['Int']['output'];
  type: MessageType;
  updatedAt: Scalars['DateTime']['output'];
};

export enum MessageType {
  ScheduleOpportunity = 'SCHEDULE_OPPORTUNITY',
  Text = 'TEXT'
}

/** GraphQL Mutation Root */
export type Mutation = {
  __typename?: 'Mutation';
  addAccount: Account;
  addCategory: Category;
  addCollection: Collection;
  addCommittee: Committee;
  addCommunity: Community;
  addItem: Item;
  addLocation: Location;
  addMedia: Media;
  addMessage: Message;
  addPledge: Pledge;
  addProfile: Profile;
  addPublish: Publish;
  addReview: Review;
  addSchedule: Schedule;
  addScheduleOpportunity: ScheduleOpportunity;
  addTransaction: Transaction;
  deleteAccount: Scalars['Boolean']['output'];
  deleteCategory: Scalars['Boolean']['output'];
  deleteCollection: Scalars['Boolean']['output'];
  /** Delete a committee */
  deleteCommittee: Scalars['Boolean']['output'];
  /** Delete a community */
  deleteCommunity: Scalars['Boolean']['output'];
  deleteItem: Scalars['Boolean']['output'];
  deleteLocation: Scalars['Boolean']['output'];
  deleteMedia: Scalars['Boolean']['output'];
  deleteProfile: Scalars['Boolean']['output'];
  deletePublish: Scalars['Boolean']['output'];
  updateCategory: Category;
  updateCollection: Collection;
  /** Update committee role */
  updateCommitteeRole: Committee;
  /** Update a community */
  updateCommunity: Community;
  updateItem: Item;
  updateLocation: Location;
  updateMedia: Media;
  updatePledge: Pledge;
  updateProfile: Profile;
  updatePublish: Publish;
  updateScheduleOpportunity: ScheduleOpportunity;
  updateTransaction: Transaction;
};


/** GraphQL Mutation Root */
export type MutationAddAccountArgs = {
  id: Scalars['UUID']['input'];
};


/** GraphQL Mutation Root */
export type MutationAddCategoryArgs = {
  description: Scalars['String']['input'];
  name: Scalars['String']['input'];
  parentId?: InputMaybe<Scalars['Int']['input']>;
};


/** GraphQL Mutation Root */
export type MutationAddCollectionArgs = {
  collectionType: CollectionType;
  description: Scalars['String']['input'];
  idProfile: Scalars['UUID']['input'];
  isPublic: Scalars['Boolean']['input'];
  name: Scalars['String']['input'];
};


/** GraphQL Mutation Root */
export type MutationAddCommitteeArgs = {
  idCommunity: Scalars['Int']['input'];
  idProfile: Scalars['UUID']['input'];
  memberRole: CommitteeRole;
};


/** GraphQL Mutation Root */
export type MutationAddCommunityArgs = {
  communityType: CommunityType;
  createdBy: Scalars['UUID']['input'];
  description: Scalars['String']['input'];
  owner: Scalars['UUID']['input'];
  title: Scalars['String']['input'];
};


/** GraphQL Mutation Root */
export type MutationAddItemArgs = {
  condition: ItemCondition;
  currency?: InputMaybe<Scalars['String']['input']>;
  description: Scalars['String']['input'];
  idCategory: Scalars['Int']['input'];
  idLocation?: InputMaybe<Scalars['Int']['input']>;
  idProfile: Scalars['UUID']['input'];
  intentAction: ItemIntentAction;
  itemType: ItemType;
  price?: InputMaybe<Scalars['Float']['input']>;
  status: ItemStatus;
  title: Scalars['String']['input'];
};


/** GraphQL Mutation Root */
export type MutationAddLocationArgs = {
  address: Scalars['String']['input'];
  city: Scalars['String']['input'];
  country: Scalars['String']['input'];
  idProfile: Scalars['UUID']['input'];
  name: Scalars['String']['input'];
  postalCode: Scalars['String']['input'];
  state: Scalars['String']['input'];
};


/** GraphQL Mutation Root */
export type MutationAddMediaArgs = {
  idItem: Scalars['Int']['input'];
  mediaType: MediaType;
  position: Scalars['Int']['input'];
  url: Scalars['String']['input'];
};


/** GraphQL Mutation Root */
export type MutationAddMessageArgs = {
  content: Scalars['String']['input'];
  idSender: Scalars['UUID']['input'];
  idTransaction: Scalars['Int']['input'];
  type: MessageType;
};


/** GraphQL Mutation Root */
export type MutationAddPledgeArgs = {
  idItem: Scalars['Int']['input'];
  idProfile: Scalars['UUID']['input'];
  intentAction: PledgeIntentAction;
  message?: InputMaybe<Scalars['String']['input']>;
  status: PledgeStatus;
};


/** GraphQL Mutation Root */
export type MutationAddProfileArgs = {
  description?: InputMaybe<Scalars['String']['input']>;
  idAccount: Scalars['UUID']['input'];
  name: Scalars['String']['input'];
  profileType?: InputMaybe<ProfileType>;
};


/** GraphQL Mutation Root */
export type MutationAddPublishArgs = {
  createdBy: Scalars['UUID']['input'];
  idCollection: Scalars['Int']['input'];
  idItem: Scalars['Int']['input'];
  note?: InputMaybe<Scalars['String']['input']>;
  position: Scalars['Int']['input'];
};


/** GraphQL Mutation Root */
export type MutationAddReviewArgs = {
  comment?: InputMaybe<Scalars['String']['input']>;
  idSubject: Scalars['Int']['input'];
  idTransaction: Scalars['Int']['input'];
  rating: Scalars['Int']['input'];
};


/** GraphQL Mutation Root */
export type MutationAddScheduleArgs = {
  scheduledFor: Scalars['DateTime']['input'];
};


/** GraphQL Mutation Root */
export type MutationAddScheduleOpportunityArgs = {
  idOpportunity: Scalars['Int']['input'];
  idSchedule: Scalars['Int']['input'];
};


/** GraphQL Mutation Root */
export type MutationAddTransactionArgs = {
  idLocation?: InputMaybe<Scalars['Int']['input']>;
  idPledge: Scalars['Int']['input'];
  idSchedule?: InputMaybe<Scalars['Int']['input']>;
  status: TransactionStatus;
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
  idProfile: Scalars['UUID']['input'];
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
  id: Scalars['UUID']['input'];
};


/** GraphQL Mutation Root */
export type MutationDeletePublishArgs = {
  idCollection: Scalars['Int']['input'];
  idItem: Scalars['Int']['input'];
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
  collectionType?: InputMaybe<CollectionType>;
  description?: InputMaybe<Scalars['String']['input']>;
  id: Scalars['Int']['input'];
  isPublic?: InputMaybe<Scalars['Boolean']['input']>;
  name?: InputMaybe<Scalars['String']['input']>;
};


/** GraphQL Mutation Root */
export type MutationUpdateCommitteeRoleArgs = {
  idCommunity: Scalars['Int']['input'];
  idProfile: Scalars['UUID']['input'];
  memberRole: CommitteeRole;
};


/** GraphQL Mutation Root */
export type MutationUpdateCommunityArgs = {
  communityType?: InputMaybe<CommunityType>;
  description?: InputMaybe<Scalars['String']['input']>;
  id: Scalars['Int']['input'];
  title?: InputMaybe<Scalars['String']['input']>;
};


/** GraphQL Mutation Root */
export type MutationUpdateItemArgs = {
  condition?: InputMaybe<ItemCondition>;
  currency?: InputMaybe<Scalars['String']['input']>;
  description?: InputMaybe<Scalars['String']['input']>;
  id: Scalars['Int']['input'];
  idCategory?: InputMaybe<Scalars['Int']['input']>;
  idLocation?: InputMaybe<Scalars['Int']['input']>;
  intentAction?: InputMaybe<ItemIntentAction>;
  itemType?: InputMaybe<ItemType>;
  price?: InputMaybe<Scalars['Float']['input']>;
  status?: InputMaybe<ItemStatus>;
  title?: InputMaybe<Scalars['String']['input']>;
};


/** GraphQL Mutation Root */
export type MutationUpdateLocationArgs = {
  address?: InputMaybe<Scalars['String']['input']>;
  city?: InputMaybe<Scalars['String']['input']>;
  country?: InputMaybe<Scalars['String']['input']>;
  id: Scalars['Int']['input'];
  name?: InputMaybe<Scalars['String']['input']>;
  postalCode?: InputMaybe<Scalars['String']['input']>;
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
export type MutationUpdatePledgeArgs = {
  id: Scalars['Int']['input'];
  status: PledgeStatus;
};


/** GraphQL Mutation Root */
export type MutationUpdateProfileArgs = {
  description?: InputMaybe<Scalars['String']['input']>;
  id: Scalars['UUID']['input'];
  name?: InputMaybe<Scalars['String']['input']>;
  profileType?: InputMaybe<ProfileType>;
};


/** GraphQL Mutation Root */
export type MutationUpdatePublishArgs = {
  idCollection: Scalars['Int']['input'];
  idItem: Scalars['Int']['input'];
  note?: InputMaybe<Scalars['String']['input']>;
  position: Scalars['Int']['input'];
  updatedBy: Scalars['UUID']['input'];
};


/** GraphQL Mutation Root */
export type MutationUpdateScheduleOpportunityArgs = {
  id: Scalars['Int']['input'];
  windowEnd: Scalars['DateTime']['input'];
  windowStart: Scalars['DateTime']['input'];
};


/** GraphQL Mutation Root */
export type MutationUpdateTransactionArgs = {
  id: Scalars['Int']['input'];
  status: TransactionStatus;
};

export type Pledge = {
  __typename?: 'Pledge';
  createdAt: Scalars['DateTime']['output'];
  id: Scalars['Int']['output'];
  idItem: Scalars['Int']['output'];
  idProfile: Scalars['UUID']['output'];
  intentAction: PledgeIntentAction;
  message?: Maybe<Scalars['String']['output']>;
  status: PledgeStatus;
  updatedAt: Scalars['DateTime']['output'];
};

export enum PledgeIntentAction {
  Give = 'GIVE',
  Receive = 'RECEIVE'
}

export enum PledgeStatus {
  Approved = 'APPROVED',
  Declined = 'DECLINED',
  Pending = 'PENDING'
}

export type Profile = {
  __typename?: 'Profile';
  createdAt: Scalars['DateTime']['output'];
  createdBy: Scalars['UUID']['output'];
  description?: Maybe<Scalars['String']['output']>;
  id: Scalars['Int']['output'];
  name?: Maybe<Scalars['String']['output']>;
  owner: Scalars['UUID']['output'];
  type?: Maybe<ProfileType>;
  updatedAt?: Maybe<Scalars['DateTime']['output']>;
};

export enum ProfileType {
  Company = 'COMPANY',
  Individual = 'INDIVIDUAL',
  Organization = 'ORGANIZATION'
}

export type Publish = {
  __typename?: 'Publish';
  addedBy: Scalars['UUID']['output'];
  createdAt: Scalars['DateTime']['output'];
  idCollection: Scalars['Int']['output'];
  idItem: Scalars['Int']['output'];
  note?: Maybe<Scalars['String']['output']>;
  position: Scalars['Int']['output'];
  updatedAt: Scalars['DateTime']['output'];
};

/** GraphQL Query Root */
export type Query = {
  __typename?: 'Query';
  /** Get account by ID */
  accountById?: Maybe<Account>;
  /** Get all accounts */
  accounts: Array<Account>;
  /** Get committee by ID */
  committeeById?: Maybe<Committee>;
  /** Get committee by profile and community */
  committeeByProfileAndCommunity?: Maybe<Committee>;
  /** Get all committees */
  committees: Array<Committee>;
  /** Get committees by community */
  committeesByCommunity: Array<Committee>;
  /** Get committees by profile */
  committeesByProfile: Array<Committee>;
  /** Get all communities */
  communities: Array<Community>;
  /** Get communities by profile */
  communitiesByProfile: Array<Community>;
  /** Get community by ID */
  communityById?: Maybe<Community>;
  dummyTest: Array<Scalars['String']['output']>;
  dummyTestRequestHeader: Scalars['String']['output'];
  dummyTestSecure: Test;
  dummyTestSecureGuard: Test;
  dummyTestSecurePermissionCheck: Test;
  getCategories: Array<Category>;
  getCategoriesByParent: Array<Category>;
  getCategoryById?: Maybe<Category>;
  getCollectionById?: Maybe<Collection>;
  getCollections: Array<Collection>;
  getCollectionsByProfile: Array<Collection>;
  getItemById?: Maybe<Item>;
  getItems: Array<Item>;
  getItemsByCategory: Array<Item>;
  getItemsByProfile: Array<Item>;
  getLocationById?: Maybe<Location>;
  getLocations: Array<Location>;
  getLocationsByProfile: Array<Location>;
  getMedia: Array<Media>;
  getMediaById?: Maybe<Media>;
  getMediaByItem: Array<Media>;
  getMessageById?: Maybe<Message>;
  getMessages: Array<Message>;
  getMessagesByTransaction: Array<Message>;
  getPledgeById?: Maybe<Pledge>;
  getPledges: Array<Pledge>;
  getPledgesByItem: Array<Pledge>;
  getPledgesByProfile: Array<Pledge>;
  getPublishById?: Maybe<Publish>;
  getPublishByItemAndCollection?: Maybe<Publish>;
  getPublishes: Array<Publish>;
  getPublishesByCollection: Array<Publish>;
  getPublishesByItem: Array<Publish>;
  getReviewById?: Maybe<Review>;
  getReviewByTransactionAndSubject?: Maybe<Review>;
  getReviews: Array<Review>;
  getScheduleById?: Maybe<Schedule>;
  getScheduleOpportunities: Array<ScheduleOpportunity>;
  getScheduleOpportunityById?: Maybe<ScheduleOpportunity>;
  getSchedules: Array<Schedule>;
  getTransactionById?: Maybe<Transaction>;
  getTransactions: Array<Transaction>;
  getTransactionsByPledge: Array<Transaction>;
  /** Get profile by account */
  profileByAccount?: Maybe<Profile>;
  /** Get profile by ID */
  profileById?: Maybe<Profile>;
  /** Get all profiles */
  profiles: Array<Profile>;
  tests: Array<Test>;
};


/** GraphQL Query Root */
export type QueryAccountByIdArgs = {
  id: Scalars['UUID']['input'];
};


/** GraphQL Query Root */
export type QueryCommitteeByIdArgs = {
  idCommunity: Scalars['Int']['input'];
  idProfile: Scalars['UUID']['input'];
};


/** GraphQL Query Root */
export type QueryCommitteeByProfileAndCommunityArgs = {
  idCommunity: Scalars['Int']['input'];
  idProfile: Scalars['UUID']['input'];
};


/** GraphQL Query Root */
export type QueryCommitteesByCommunityArgs = {
  idCommunity: Scalars['Int']['input'];
};


/** GraphQL Query Root */
export type QueryCommitteesByProfileArgs = {
  idProfile: Scalars['UUID']['input'];
};


/** GraphQL Query Root */
export type QueryCommunitiesByProfileArgs = {
  idProfile: Scalars['UUID']['input'];
};


/** GraphQL Query Root */
export type QueryCommunityByIdArgs = {
  id: Scalars['Int']['input'];
};


/** GraphQL Query Root */
export type QueryGetCategoriesByParentArgs = {
  parentId?: InputMaybe<Scalars['Int']['input']>;
};


/** GraphQL Query Root */
export type QueryGetCategoryByIdArgs = {
  id: Scalars['Int']['input'];
};


/** GraphQL Query Root */
export type QueryGetCollectionByIdArgs = {
  id: Scalars['Int']['input'];
};


/** GraphQL Query Root */
export type QueryGetCollectionsByProfileArgs = {
  profileId: Scalars['UUID']['input'];
};


/** GraphQL Query Root */
export type QueryGetItemByIdArgs = {
  id: Scalars['Int']['input'];
};


/** GraphQL Query Root */
export type QueryGetItemsByCategoryArgs = {
  categoryId: Scalars['Int']['input'];
};


/** GraphQL Query Root */
export type QueryGetItemsByProfileArgs = {
  profileId: Scalars['UUID']['input'];
};


/** GraphQL Query Root */
export type QueryGetLocationByIdArgs = {
  id: Scalars['Int']['input'];
};


/** GraphQL Query Root */
export type QueryGetLocationsByProfileArgs = {
  profileId: Scalars['UUID']['input'];
};


/** GraphQL Query Root */
export type QueryGetMediaByIdArgs = {
  id: Scalars['Int']['input'];
};


/** GraphQL Query Root */
export type QueryGetMediaByItemArgs = {
  itemId: Scalars['Int']['input'];
};


/** GraphQL Query Root */
export type QueryGetMessageByIdArgs = {
  id: Scalars['Int']['input'];
};


/** GraphQL Query Root */
export type QueryGetMessagesByTransactionArgs = {
  idTransaction: Scalars['Int']['input'];
};


/** GraphQL Query Root */
export type QueryGetPledgeByIdArgs = {
  id: Scalars['Int']['input'];
};


/** GraphQL Query Root */
export type QueryGetPledgesByItemArgs = {
  idItem: Scalars['Int']['input'];
};


/** GraphQL Query Root */
export type QueryGetPledgesByProfileArgs = {
  idProfile: Scalars['UUID']['input'];
};


/** GraphQL Query Root */
export type QueryGetPublishByIdArgs = {
  id: Scalars['Int']['input'];
};


/** GraphQL Query Root */
export type QueryGetPublishByItemAndCollectionArgs = {
  collectionId: Scalars['Int']['input'];
  itemId: Scalars['Int']['input'];
};


/** GraphQL Query Root */
export type QueryGetPublishesByCollectionArgs = {
  collectionId: Scalars['Int']['input'];
};


/** GraphQL Query Root */
export type QueryGetPublishesByItemArgs = {
  itemId: Scalars['Int']['input'];
};


/** GraphQL Query Root */
export type QueryGetReviewByIdArgs = {
  idSubject: Scalars['Int']['input'];
  idTransaction: Scalars['Int']['input'];
};


/** GraphQL Query Root */
export type QueryGetReviewByTransactionAndSubjectArgs = {
  idSubject: Scalars['Int']['input'];
  idTransaction: Scalars['Int']['input'];
};


/** GraphQL Query Root */
export type QueryGetScheduleByIdArgs = {
  id: Scalars['Int']['input'];
};


/** GraphQL Query Root */
export type QueryGetScheduleOpportunityByIdArgs = {
  id: Scalars['Int']['input'];
};


/** GraphQL Query Root */
export type QueryGetTransactionByIdArgs = {
  id: Scalars['Int']['input'];
};


/** GraphQL Query Root */
export type QueryGetTransactionsByPledgeArgs = {
  idPledge: Scalars['Int']['input'];
};


/** GraphQL Query Root */
export type QueryProfileByAccountArgs = {
  idAccount: Scalars['UUID']['input'];
};


/** GraphQL Query Root */
export type QueryProfileByIdArgs = {
  id: Scalars['UUID']['input'];
};

export type Review = {
  __typename?: 'Review';
  comment?: Maybe<Scalars['String']['output']>;
  createdAt: Scalars['DateTime']['output'];
  idSubjectProfile: Scalars['UUID']['output'];
  idTransaction: Scalars['Int']['output'];
  rating: Scalars['Int']['output'];
  updatedAt: Scalars['DateTime']['output'];
};

export type Schedule = {
  __typename?: 'Schedule';
  createdAt: Scalars['DateTime']['output'];
  id: Scalars['Int']['output'];
  scheduledFor: Scalars['DateTime']['output'];
  updatedAt: Scalars['DateTime']['output'];
};

export type ScheduleOpportunity = {
  __typename?: 'ScheduleOpportunity';
  createdAt: Scalars['DateTime']['output'];
  id: Scalars['Int']['output'];
  updatedAt: Scalars['DateTime']['output'];
  windowEnd: Scalars['DateTime']['output'];
  windowStart: Scalars['DateTime']['output'];
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
  updatedAt: Scalars['DateTime']['output'];
};

export enum TransactionStatus {
  Cancelled = 'CANCELLED',
  Completed = 'COMPLETED',
  InProgress = 'IN_PROGRESS'
}

export type GetTestListPartialQueryVariables = Exact<{ [key: string]: never; }>;


export type GetTestListPartialQuery = { __typename?: 'Query', tests: Array<{ __typename?: 'Test', i: number, d: Date }> };

export type AddAccountMutationVariables = Exact<{
  id: Scalars['UUID']['input'];
}>;


export type AddAccountMutation = { __typename?: 'Mutation', addAccount: { __typename?: 'Account', id: string, remarks?: string | null, createdAt: Date } };

export type AddProfileMutationVariables = Exact<{
  idAccount: Scalars['UUID']['input'];
  name: Scalars['String']['input'];
  description?: InputMaybe<Scalars['String']['input']>;
  profileType?: InputMaybe<ProfileType>;
}>;


export type AddProfileMutation = { __typename?: 'Mutation', addProfile: { __typename?: 'Profile', id: number, name?: string | null, description?: string | null, type?: ProfileType | null, owner: string, createdAt: Date, updatedAt?: Date | null, createdBy: string } };

export type AddCommunityMutationVariables = Exact<{
  title: Scalars['String']['input'];
  description: Scalars['String']['input'];
  communityType: CommunityType;
  owner: Scalars['UUID']['input'];
  createdBy: Scalars['UUID']['input'];
}>;


export type AddCommunityMutation = { __typename?: 'Mutation', addCommunity: { __typename?: 'Community', id: number, title?: string | null, description?: string | null, type: CommunityType, owner: string, createdAt: Date, updatedAt?: Date | null, createdBy: string } };

export type AddCommitteeMutationVariables = Exact<{
  idCommunity: Scalars['Int']['input'];
  idProfile: Scalars['UUID']['input'];
  memberRole: CommitteeRole;
}>;


export type AddCommitteeMutation = { __typename?: 'Mutation', addCommittee: { __typename?: 'Committee', idCommunity: number, idProfile: number, memberRole: CommitteeRole, joinedAt: Date } };

export type AddCategoryMutationVariables = Exact<{
  name: Scalars['String']['input'];
  description: Scalars['String']['input'];
  parentId?: InputMaybe<Scalars['Int']['input']>;
}>;


export type AddCategoryMutation = { __typename?: 'Mutation', addCategory: { __typename?: 'Category', id: number, title: string, description?: string | null, categoryParent?: number | null, createdAt: Date, updatedAt: Date } };

export type AddLocationMutationVariables = Exact<{
  name: Scalars['String']['input'];
  address: Scalars['String']['input'];
  city: Scalars['String']['input'];
  state: Scalars['String']['input'];
  country: Scalars['String']['input'];
  postalCode: Scalars['String']['input'];
  idProfile: Scalars['UUID']['input'];
}>;


export type AddLocationMutation = { __typename?: 'Mutation', addLocation: { __typename?: 'Location', id: number, addressLine1: string, addressLine2?: string | null, city: string, state: string, country: string, district?: string | null, geom?: string | null, entranceNote?: string | null, createdAt: Date, updatedAt: Date } };

export type AddItemMutationVariables = Exact<{
  title: Scalars['String']['input'];
  description: Scalars['String']['input'];
  idCategory: Scalars['Int']['input'];
  idProfile: Scalars['UUID']['input'];
  idLocation?: InputMaybe<Scalars['Int']['input']>;
  price?: InputMaybe<Scalars['Float']['input']>;
  currency?: InputMaybe<Scalars['String']['input']>;
  itemType: ItemType;
  intentAction: ItemIntentAction;
  status: ItemStatus;
  condition: ItemCondition;
}>;


export type AddItemMutation = { __typename?: 'Mutation', addItem: { __typename?: 'Item', id: number, title: string, description?: string | null, type: ItemType, intentAction: ItemIntentAction, status: ItemStatus, condition: ItemCondition, createdAt: Date, updatedAt: Date } };

export type AddMediaMutationVariables = Exact<{
  idItem: Scalars['Int']['input'];
  url: Scalars['String']['input'];
  mediaType: MediaType;
  position: Scalars['Int']['input'];
}>;


export type AddMediaMutation = { __typename?: 'Mutation', addMedia: { __typename?: 'Media', id: number, url: string, type: MediaType, caption?: string | null, createdAt: Date, updatedAt: Date } };

export type AddCollectionMutationVariables = Exact<{
  name: Scalars['String']['input'];
  description: Scalars['String']['input'];
  idProfile: Scalars['UUID']['input'];
  isPublic: Scalars['Boolean']['input'];
  collectionType: CollectionType;
}>;


export type AddCollectionMutation = { __typename?: 'Mutation', addCollection: { __typename?: 'Collection', id: number, title: string, type: CollectionType, visibility: CollectionVisibility, position: number, createdAt: Date, updatedAt: Date } };

export type AddPledgeMutationVariables = Exact<{
  idProfile: Scalars['UUID']['input'];
  idItem: Scalars['Int']['input'];
  intentAction: PledgeIntentAction;
  message?: InputMaybe<Scalars['String']['input']>;
  status: PledgeStatus;
}>;


export type AddPledgeMutation = { __typename?: 'Mutation', addPledge: { __typename?: 'Pledge', id: number, idProfile: string, idItem: number, intentAction: PledgeIntentAction, message?: string | null, status: PledgeStatus, createdAt: Date, updatedAt: Date } };

export type AddTransactionMutationVariables = Exact<{
  idPledge: Scalars['Int']['input'];
  status: TransactionStatus;
  idSchedule?: InputMaybe<Scalars['Int']['input']>;
  idLocation?: InputMaybe<Scalars['Int']['input']>;
}>;


export type AddTransactionMutation = { __typename?: 'Mutation', addTransaction: { __typename?: 'Transaction', id: number, idPledge: number, status: TransactionStatus, createdAt: Date, updatedAt: Date } };

export type AddPublishMutationVariables = Exact<{
  idItem: Scalars['Int']['input'];
  idCollection: Scalars['Int']['input'];
  note?: InputMaybe<Scalars['String']['input']>;
  position: Scalars['Int']['input'];
  createdBy: Scalars['UUID']['input'];
}>;


export type AddPublishMutation = { __typename?: 'Mutation', addPublish: { __typename?: 'Publish', idItem: number, idCollection: number, note?: string | null, position: number, addedBy: string, createdAt: Date, updatedAt: Date } };

export type AddMessageMutationVariables = Exact<{
  idTransaction: Scalars['Int']['input'];
  idSender: Scalars['UUID']['input'];
  type: MessageType;
  content: Scalars['String']['input'];
}>;


export type AddMessageMutation = { __typename?: 'Mutation', addMessage: { __typename?: 'Message', id: number, idSender: string, idTransaction: number, type: MessageType, content: string, createdAt: Date, updatedAt: Date } };

export type GetAccountsQueryVariables = Exact<{ [key: string]: never; }>;


export type GetAccountsQuery = { __typename?: 'Query', accounts: Array<{ __typename?: 'Account', id: string, createdAt: Date, remarks?: string | null }> };

export type GetProfilesQueryVariables = Exact<{ [key: string]: never; }>;


export type GetProfilesQuery = { __typename?: 'Query', profiles: Array<{ __typename?: 'Profile', id: number, name?: string | null, type?: ProfileType | null, description?: string | null, createdAt: Date, updatedAt?: Date | null }> };

export type GetCommunitiesQueryVariables = Exact<{ [key: string]: never; }>;


export type GetCommunitiesQuery = { __typename?: 'Query', communities: Array<{ __typename?: 'Community', id: number, title?: string | null, description?: string | null, type: CommunityType, createdAt: Date, updatedAt?: Date | null }> };

export type GetCommitteesByCommunityQueryVariables = Exact<{
  idCommunity: Scalars['Int']['input'];
}>;


export type GetCommitteesByCommunityQuery = { __typename?: 'Query', committeesByCommunity: Array<{ __typename?: 'Committee', idCommunity: number, idProfile: number, memberRole: CommitteeRole, joinedAt: Date }> };

export type GetCategoriesQueryVariables = Exact<{ [key: string]: never; }>;


export type GetCategoriesQuery = { __typename?: 'Query', getCategories: Array<{ __typename?: 'Category', id: number, title: string, description?: string | null, categoryParent?: number | null, createdAt: Date, updatedAt: Date }> };

export type GetItemsByCategoryQueryVariables = Exact<{
  categoryId: Scalars['Int']['input'];
}>;


export type GetItemsByCategoryQuery = { __typename?: 'Query', getItemsByCategory: Array<{ __typename?: 'Item', id: number, title: string, description?: string | null, status: ItemStatus, type: ItemType, condition: ItemCondition, intentAction: ItemIntentAction, quantity: number, createdAt: Date, updatedAt: Date }> };

export type GetLocationsQueryVariables = Exact<{ [key: string]: never; }>;


export type GetLocationsQuery = { __typename?: 'Query', getLocations: Array<{ __typename?: 'Location', id: number, addressLine1: string, addressLine2?: string | null, city: string, state: string, country: string, district?: string | null, entranceNote?: string | null, geom?: string | null, createdAt: Date, updatedAt: Date }> };

export type GetCollectionsQueryVariables = Exact<{ [key: string]: never; }>;


export type GetCollectionsQuery = { __typename?: 'Query', getCollections: Array<{ __typename?: 'Collection', id: number, title: string, type: CollectionType, visibility: CollectionVisibility, position: number, createdAt: Date, updatedAt: Date }> };

export type GetPublishesByCollectionQueryVariables = Exact<{
  collectionId: Scalars['Int']['input'];
}>;


export type GetPublishesByCollectionQuery = { __typename?: 'Query', getPublishesByCollection: Array<{ __typename?: 'Publish', idCollection: number, idItem: number, position: number, note?: string | null, addedBy: string, createdAt: Date, updatedAt: Date }> };

export type GetItemByIdQueryVariables = Exact<{
  id: Scalars['Int']['input'];
}>;


export type GetItemByIdQuery = { __typename?: 'Query', getItemById?: { __typename?: 'Item', id: number, title: string, description?: string | null, status: ItemStatus, type: ItemType, condition: ItemCondition, intentAction: ItemIntentAction, quantity: number, createdAt: Date, updatedAt: Date } | null };

export type GetItemsQueryVariables = Exact<{ [key: string]: never; }>;


export type GetItemsQuery = { __typename?: 'Query', getItems: Array<{ __typename?: 'Item', id: number, title: string, description?: string | null, status: ItemStatus, type: ItemType, condition: ItemCondition, intentAction: ItemIntentAction, quantity: number, createdAt: Date, updatedAt: Date }> };

export type GetMediaByItemQueryVariables = Exact<{
  itemId: Scalars['Int']['input'];
}>;


export type GetMediaByItemQuery = { __typename?: 'Query', getMediaByItem: Array<{ __typename?: 'Media', id: number, url: string, type: MediaType, caption?: string | null, createdAt: Date, updatedAt: Date }> };

export type GetAccountListQueryVariables = Exact<{ [key: string]: never; }>;


export type GetAccountListQuery = { __typename?: 'Query', accounts: Array<{ __typename?: 'Account', id: string, createdAt: Date }> };

export type GetTestListQueryVariables = Exact<{ [key: string]: never; }>;


export type GetTestListQuery = { __typename?: 'Query', tests: Array<{ __typename?: 'Test', i: number, s: string, d: Date }> };

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
  tests {
    i
    d
  }
}
    `) as unknown as TypedDocumentString<GetTestListPartialQuery, GetTestListPartialQueryVariables>;
export const AddAccountDocument = new TypedDocumentString(`
    mutation AddAccount($id: UUID!) {
  addAccount(id: $id) {
    id
    remarks
    createdAt
  }
}
    `) as unknown as TypedDocumentString<AddAccountMutation, AddAccountMutationVariables>;
export const AddProfileDocument = new TypedDocumentString(`
    mutation AddProfile($idAccount: UUID!, $name: String!, $description: String, $profileType: ProfileType) {
  addProfile(
    idAccount: $idAccount
    name: $name
    description: $description
    profileType: $profileType
  ) {
    id
    name
    description
    type
    owner
    createdAt
    updatedAt
    createdBy
  }
}
    `) as unknown as TypedDocumentString<AddProfileMutation, AddProfileMutationVariables>;
export const AddCommunityDocument = new TypedDocumentString(`
    mutation AddCommunity($title: String!, $description: String!, $communityType: CommunityType!, $owner: UUID!, $createdBy: UUID!) {
  addCommunity(
    title: $title
    description: $description
    communityType: $communityType
    owner: $owner
    createdBy: $createdBy
  ) {
    id
    title
    description
    type
    owner
    createdAt
    updatedAt
    createdBy
  }
}
    `) as unknown as TypedDocumentString<AddCommunityMutation, AddCommunityMutationVariables>;
export const AddCommitteeDocument = new TypedDocumentString(`
    mutation AddCommittee($idCommunity: Int!, $idProfile: UUID!, $memberRole: CommitteeRole!) {
  addCommittee(
    idCommunity: $idCommunity
    idProfile: $idProfile
    memberRole: $memberRole
  ) {
    idCommunity
    idProfile
    memberRole
    joinedAt
  }
}
    `) as unknown as TypedDocumentString<AddCommitteeMutation, AddCommitteeMutationVariables>;
export const AddCategoryDocument = new TypedDocumentString(`
    mutation AddCategory($name: String!, $description: String!, $parentId: Int) {
  addCategory(name: $name, description: $description, parentId: $parentId) {
    id
    title
    description
    categoryParent
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<AddCategoryMutation, AddCategoryMutationVariables>;
export const AddLocationDocument = new TypedDocumentString(`
    mutation AddLocation($name: String!, $address: String!, $city: String!, $state: String!, $country: String!, $postalCode: String!, $idProfile: UUID!) {
  addLocation(
    name: $name
    address: $address
    city: $city
    state: $state
    country: $country
    postalCode: $postalCode
    idProfile: $idProfile
  ) {
    id
    addressLine1
    addressLine2
    city
    state
    country
    district
    geom
    entranceNote
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<AddLocationMutation, AddLocationMutationVariables>;
export const AddItemDocument = new TypedDocumentString(`
    mutation AddItem($title: String!, $description: String!, $idCategory: Int!, $idProfile: UUID!, $idLocation: Int, $price: Float, $currency: String, $itemType: ItemType!, $intentAction: ItemIntentAction!, $status: ItemStatus!, $condition: ItemCondition!) {
  addItem(
    title: $title
    description: $description
    idCategory: $idCategory
    idProfile: $idProfile
    idLocation: $idLocation
    price: $price
    currency: $currency
    itemType: $itemType
    intentAction: $intentAction
    status: $status
    condition: $condition
  ) {
    id
    title
    description
    type
    intentAction
    status
    condition
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<AddItemMutation, AddItemMutationVariables>;
export const AddMediaDocument = new TypedDocumentString(`
    mutation AddMedia($idItem: Int!, $url: String!, $mediaType: MediaType!, $position: Int!) {
  addMedia(idItem: $idItem, url: $url, mediaType: $mediaType, position: $position) {
    id
    url
    type
    caption
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<AddMediaMutation, AddMediaMutationVariables>;
export const AddCollectionDocument = new TypedDocumentString(`
    mutation AddCollection($name: String!, $description: String!, $idProfile: UUID!, $isPublic: Boolean!, $collectionType: CollectionType!) {
  addCollection(
    name: $name
    description: $description
    idProfile: $idProfile
    isPublic: $isPublic
    collectionType: $collectionType
  ) {
    id
    title
    type
    visibility
    position
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<AddCollectionMutation, AddCollectionMutationVariables>;
export const AddPledgeDocument = new TypedDocumentString(`
    mutation AddPledge($idProfile: UUID!, $idItem: Int!, $intentAction: PledgeIntentAction!, $message: String, $status: PledgeStatus!) {
  addPledge(
    idProfile: $idProfile
    idItem: $idItem
    intentAction: $intentAction
    message: $message
    status: $status
  ) {
    id
    idProfile
    idItem
    intentAction
    message
    status
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<AddPledgeMutation, AddPledgeMutationVariables>;
export const AddTransactionDocument = new TypedDocumentString(`
    mutation AddTransaction($idPledge: Int!, $status: TransactionStatus!, $idSchedule: Int, $idLocation: Int) {
  addTransaction(
    idPledge: $idPledge
    status: $status
    idSchedule: $idSchedule
    idLocation: $idLocation
  ) {
    id
    idPledge
    status
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<AddTransactionMutation, AddTransactionMutationVariables>;
export const AddPublishDocument = new TypedDocumentString(`
    mutation AddPublish($idItem: Int!, $idCollection: Int!, $note: String, $position: Int!, $createdBy: UUID!) {
  addPublish(
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
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<AddPublishMutation, AddPublishMutationVariables>;
export const AddMessageDocument = new TypedDocumentString(`
    mutation AddMessage($idTransaction: Int!, $idSender: UUID!, $type: MessageType!, $content: String!) {
  addMessage(
    idTransaction: $idTransaction
    idSender: $idSender
    type: $type
    content: $content
  ) {
    id
    idSender
    idTransaction
    type
    content
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<AddMessageMutation, AddMessageMutationVariables>;
export const GetAccountsDocument = new TypedDocumentString(`
    query GetAccounts {
  accounts {
    id
    createdAt
    remarks
  }
}
    `) as unknown as TypedDocumentString<GetAccountsQuery, GetAccountsQueryVariables>;
export const GetProfilesDocument = new TypedDocumentString(`
    query GetProfiles {
  profiles {
    id
    name
    type
    description
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<GetProfilesQuery, GetProfilesQueryVariables>;
export const GetCommunitiesDocument = new TypedDocumentString(`
    query GetCommunities {
  communities {
    id
    title
    description
    type
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<GetCommunitiesQuery, GetCommunitiesQueryVariables>;
export const GetCommitteesByCommunityDocument = new TypedDocumentString(`
    query GetCommitteesByCommunity($idCommunity: Int!) {
  committeesByCommunity(idCommunity: $idCommunity) {
    idCommunity
    idProfile
    memberRole
    joinedAt
  }
}
    `) as unknown as TypedDocumentString<GetCommitteesByCommunityQuery, GetCommitteesByCommunityQueryVariables>;
export const GetCategoriesDocument = new TypedDocumentString(`
    query GetCategories {
  getCategories {
    id
    title
    description
    categoryParent
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<GetCategoriesQuery, GetCategoriesQueryVariables>;
export const GetItemsByCategoryDocument = new TypedDocumentString(`
    query GetItemsByCategory($categoryId: Int!) {
  getItemsByCategory(categoryId: $categoryId) {
    id
    title
    description
    status
    type
    condition
    intentAction
    quantity
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<GetItemsByCategoryQuery, GetItemsByCategoryQueryVariables>;
export const GetLocationsDocument = new TypedDocumentString(`
    query GetLocations {
  getLocations {
    id
    addressLine1
    addressLine2
    city
    state
    country
    district
    entranceNote
    geom
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<GetLocationsQuery, GetLocationsQueryVariables>;
export const GetCollectionsDocument = new TypedDocumentString(`
    query GetCollections {
  getCollections {
    id
    title
    type
    visibility
    position
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<GetCollectionsQuery, GetCollectionsQueryVariables>;
export const GetPublishesByCollectionDocument = new TypedDocumentString(`
    query GetPublishesByCollection($collectionId: Int!) {
  getPublishesByCollection(collectionId: $collectionId) {
    idCollection
    idItem
    position
    note
    addedBy
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<GetPublishesByCollectionQuery, GetPublishesByCollectionQueryVariables>;
export const GetItemByIdDocument = new TypedDocumentString(`
    query GetItemById($id: Int!) {
  getItemById(id: $id) {
    id
    title
    description
    status
    type
    condition
    intentAction
    quantity
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<GetItemByIdQuery, GetItemByIdQueryVariables>;
export const GetItemsDocument = new TypedDocumentString(`
    query GetItems {
  getItems {
    id
    title
    description
    status
    type
    condition
    intentAction
    quantity
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<GetItemsQuery, GetItemsQueryVariables>;
export const GetMediaByItemDocument = new TypedDocumentString(`
    query GetMediaByItem($itemId: Int!) {
  getMediaByItem(itemId: $itemId) {
    id
    url
    type
    caption
    createdAt
    updatedAt
  }
}
    `) as unknown as TypedDocumentString<GetMediaByItemQuery, GetMediaByItemQueryVariables>;
export const GetAccountListDocument = new TypedDocumentString(`
    query GetAccountList {
  accounts {
    id
    createdAt
  }
}
    `) as unknown as TypedDocumentString<GetAccountListQuery, GetAccountListQueryVariables>;
export const GetTestListDocument = new TypedDocumentString(`
    query GetTestList {
  tests {
    i
    s
    d
  }
}
    `) as unknown as TypedDocumentString<GetTestListQuery, GetTestListQueryVariables>;