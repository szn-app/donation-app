import { useQuery } from '@tanstack/react-query'
import {graphql} from "@/graphql-generated/gql"

const allFilmsWithVariablesQueryDocument = graphql(/* GraphQL */ `
    query Dummy1 {
      __typename
    }
  `);
  

  // TODO: 
export function f() {
    // example https://studio.apollographql.com/public/SpaceX-pxxbxen/variant/current/home

  // const { data } = useQuery({
  //   queryKey: ['films'],
  //   queryFn: () => execute(GetAccountList)
  // })
 
  console.log('data');
}