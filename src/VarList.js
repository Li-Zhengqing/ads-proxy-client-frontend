import './App.css';
import React from 'react';
import { Refresh } from "@mui/icons-material";

import { Button, Stack, List, ListItem, Container } from '@mui/material';

import DataTable from './DataTable';

class VarList extends React.Component {
  constructor (props) {
    super(props);
  }

  render() {
    return (
      <Container>
        <Stack direction="column">
          <Button variant="contained" onClick={this.props.refreshVarList}>
            <Refresh></Refresh>
          </Button>
          <DataTable rows={this.props.var_list_rows}>
          </DataTable>
        </Stack>
      </Container>
    )
  }
}

export default VarList;
