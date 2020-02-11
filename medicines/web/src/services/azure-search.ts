import fetch, { Response } from 'node-fetch';
import { buildFuzzyQuery } from './keyword';

const searchApiVersion = process.env.AZURE_SEARCH_API_VERSION;
const searchIndex = process.env.AZURE_SEARCH_INDEX;
const searchKey = process.env.AZURE_SEARCH_KEY;
const searchScoringProfile = process.env.AZURE_SEARCH_SCORING_PROFILE;
const searchService = process.env.AZURE_SEARCH_SERVICE;

export enum DocType {
  Par = 'Par',
  Pil = 'Pil',
  Spc = 'Spc',
}

export interface ISearchResult {
  '@search.highlights': { content: string[] };
  '@search.score': number;
  author: string | null;
  created: string | null;
  doc_type: DocType;
  file_name: string | null;
  keywords: string | null;
  metadata_storage_name: string;
  metadata_storage_path: string;
  metadata_storage_size: number;
  product_name: string;
  release_state: string | null;
  substance_name: string[];
  suggestions: string[];
  title: string | null;
}

export interface ISearchResults {
  resultCount: number;
  results: ISearchResult[];
}

const calculatePageStartRecord = (page: number, pageSize: number): number =>
  pageSize * (page - 1);

const buildSearchUrl = (
  query: string,
  page: number,
  pageSize: number,
  filters: ISearchFilters,
): string => {
  const url = buildBaseUrl();
  url.searchParams.append('highlight', 'content');
  url.searchParams.append('queryType', 'full');
  url.searchParams.append('$count', 'true');
  url.searchParams.append('$top', `${pageSize}`);
  url.searchParams.append(
    '$skip',
    `${calculatePageStartRecord(page, pageSize)}`,
  );
  url.searchParams.append('search', query);
  url.searchParams.append('scoringProfile', searchScoringProfile as string);
  url.searchParams.append('searchMode', 'all');
  addFilterParameter(url, filters);

  return url.toString();
};

const addFilterParameter = (url: URL, filters: ISearchFilters) => {
  const filterParameter = createFilter(filters);
  if (filterParameter.length > 0) {
    url.searchParams.append('$filter', filterParameter);
  }
};

export interface IFacetResult {
  facets: Array<{ count: number; value: string }>;
}

const buildBaseUrl = () => {
  const url = new URL(
    `https://${searchService}.search.windows.net/indexes/${searchIndex}/docs`,
  );

  url.searchParams.append('api-key', searchKey as string);
  url.searchParams.append('api-version', searchApiVersion as string);
  return url;
};

const buildFacetUrl = (query: string): string => {
  const url = buildBaseUrl();
  url.searchParams.append('facet', 'facets,count:50000,sort:value');
  url.searchParams.append('$filter', `facets/any(f: f eq '${query}')`);
  url.searchParams.append('$top', '0');
  url.searchParams.append('searchMode', 'all');

  return url.toString();
};

const getJson = async (url: string): Promise<any> => {
  const resp: Response = await fetch(url, {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
    },
  });

  if (resp.ok) {
    return resp.json();
  }
};

export interface ISearchFilters {
  docType: DocType[];
  substanceName?: string;
  productName?: string;
  sortOrder: string;
}

interface ISearchQuery {
  query: string;
  page: number;
  pageSize: number;
  filters: ISearchFilters;
}

export const docSearch = async (
  query: ISearchQuery,
): Promise<ISearchResults> => {
  const body = await getJson(
    buildSearchUrl(
      buildFuzzyQuery(query.query),
      query.page,
      query.pageSize,
      query.filters,
    ),
  );
  return {
    resultCount: body['@odata.count'],
    results: body.value,
  };
};

export const facetSearch = async (
  query: string,
): Promise<[string, IFacetResult]> => {
  const body = await getJson(buildFacetUrl(query));

  return [query, body['@search.facets']];
};

const createFilter = (filters: ISearchFilters) => {
  const filterParams: string[] = [];
  if (filters.docType) {
    const docTypeFilters = [];
    for (const docType of filters.docType) {
      docTypeFilters.push(`doc_type eq '${docType}'`);
    }
    filterParams.push(docTypeFilters.join(' or '));
  }
  if (filters.substanceName) {
    filterParams.push(
      `substance_name/any(substance: substance eq '${filters.substanceName.toUpperCase()}')`,
    );
  }
  if (filters.productName) {
    filterParams.push(`product_name eq '${filters.productName.toUpperCase()}'`);
  }
  return filterParams.join(' and ');
};
