import React from 'react';
import { Header } from 'semantic-ui-react';
import { Status } from '../../components/status/Status';
import './Encoder.css';

export class Encoder extends React.Component {
  render() {
    return (
      <>
        <Header as="h4" color="orange" textAlign="center">
          Encoder
        </Header>
        <Status />
      </>
    );
  }
}
