import { useQuery } from "react-query";
import { useParams } from "react-router-dom";
import CommitsBlock from "../../components/commits/CommitBlock";
import DiffBlock from "../../components/diffs/DiffBlock";
import { diffCommits, readCommit } from "../../services/StorageService";

export default function DiffPage(): JSX.Element {

  const { user, source, commit } = useParams(); 
  const {isLoading: commitIsLoading, data: commitData} = useQuery(["commit", user, source, commit], () => readCommit({user, source, commit}),
  {
    enabled: !!user && !!source && !!commit
  });
  const {isLoading: diffsAreLoading, data: diffs} = useQuery(["diffs", user, source, commitData], () => diffCommits({user, source, commit: commitData?.sha1, parentCommit: commitData?.parents[0] as string}),
  {
    enabled: !!user && !!source && !!commitData
  });


  return (
    <>
        {!commitIsLoading && !!commitData && 
          <CommitsBlock showParent={true} commit={commitData}/>
        }
        {!diffsAreLoading && !!diffs && 
          diffs.map((diff, index) => (
            <DiffBlock key={index} diff={diff}/>
          ))
        }
    </>
  );
}