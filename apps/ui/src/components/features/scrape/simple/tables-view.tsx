import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Stack } from '@/components/ui/stack'
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'
import type { TPageData } from './page-data'

export function TablesView({ tables }: { tables: NonNullable<TPageData['tables']> }) {
  return (
    <Stack gap="md">
      {tables.map((rows, tableIndex) => {
        const [head = [], ...body] = rows
        return (
          <Card key={tableIndex} size="sm">
            <CardHeader>
              <CardTitle>
                Table {tableIndex + 1} · {body.length} rows
              </CardTitle>
            </CardHeader>
            <CardContent>
              <Table>
                <TableHeader>
                  <TableRow>
                    {head.map((cell, index) => (
                      <TableHead key={index}>{cell}</TableHead>
                    ))}
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {body.map((row, rowIndex) => (
                    <TableRow key={rowIndex}>
                      {row.map((cell, index) => (
                        <TableCell key={index}>{cell}</TableCell>
                      ))}
                    </TableRow>
                  ))}
                </TableBody>
              </Table>
            </CardContent>
          </Card>
        )
      })}
    </Stack>
  )
}
