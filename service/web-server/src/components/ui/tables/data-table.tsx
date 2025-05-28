import React from "react";

interface Column {
  header: string;
  accessorKey: string;
}

interface DataTableProps {
  columns: Column[];
  data: Record<string, any>[];
}

function classNames(...classes: string[]) {
  return classes.filter(Boolean).join(" ");
}

export function DataTable({ columns, data }: DataTableProps) {
  return (
    <div className="container mx-auto px-3">
      <div className="mt-3 flow-root">
        <div className="-mx-3 -my-2 overflow-x-auto sm:-mx-3 lg:-mx-3">
          <div className="inline-block min-w-full py-2 align-middle">
            <table className="min-w-full border-separate border-spacing-0">
              <thead>
                <tr>
                  {columns.map((column) => (
                    <th
                      key={column.accessorKey}
                      scope="col"
                      className="sticky top-0 z-10 border-b border-gray-300 bg-white bg-opacity-75 py-3.5 pl-4 pr-3 text-left text-sm font-semibold text-gray-900 backdrop-blur backdrop-filter sm:pl-6 lg:pl-8"
                    >
                      {column.header}
                    </th>
                  ))}
                </tr>
              </thead>
              <tbody>
                {data.map((row, rowIdx) => (
                  <tr key={rowIdx}>
                    {columns.map((column) => (
                      <td
                        key={`${rowIdx}-${column.accessorKey}`}
                        className={classNames(
                          rowIdx !== data.length - 1 ? "border-b border-gray-200" : "",
                          "whitespace-nowrap py-4 pl-4 pr-3 text-sm text-gray-900 sm:pl-6 lg:pl-8"
                        )}
                      >
                        {row[column.accessorKey]}
                      </td>
                    ))}
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div>
  );
} 