import { useQuery } from '@tanstack/react-query'
import { execute } from "graphql";
import {graphql} from "@/graphql-generated"

const a = graphql(`query GetAccountList { 
    accounts { 
        id
        createdAt
    }
}`);

export function f() {
  const { data } = useQuery({
    queryKey: ['films'],
    queryFn: () => execute(GetAccountList)
  })
 
  console.log(data);
}