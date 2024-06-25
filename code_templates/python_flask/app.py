import asyncio
import io
from flask import Flask, request, jsonify, make_response
from flask_sqlalchemy import SQLAlchemy
from flask_cors import CORS, cross_origin
import logging
from sqlalchemy.orm import relationship
from os import environ

app = Flask(__name__)
cors = CORS(app)
app.config['SQLALCHEMY_DATABASE_URI'] = 'postgresql://postgres:123@localhost:5432/rust'
app.config['SQLALCHEMY_TRACK_MODIFICATIONS'] = False
db = SQLAlchemy(app)

# Entities
class Stock(db.Model):
    __tablename__ = 'stocks'

    stock_id = db.Column(db.Integer, primary_key=True, autoincrement=True)
    symbol = db.Column(db.String(30))
    name = db.Column(db.String(100))
    exchange = db.Column(db.String(50))
    sector = db.Column(db.String(50))
    industry = db.Column(db.String(50))

    # One-to-many relationship with StockPrice
    stockPrices = relationship("StockPrice", back_populates="stock")

    def __repr__(self):
        return f'<stock: {self.name}, {self.symbol}>'
    
class StockPrice(db.Model):
    __tablename__ = 'stock_prices'

    price_id = db.Column(db.Integer, primary_key=True, autoincrement=True)
    stock_id = db.Column(db.Integer, db.ForeignKey('stocks.stock_id', ondelete='CASCADE'))
    date = db.Column(db.DateTime, nullable=False)
    open = db.Column(db.Numeric(10, 4), nullable=False)
    high = db.Column(db.Numeric(10, 4), nullable=False)
    low = db.Column(db.Numeric(10, 4), nullable=False)
    close = db.Column(db.Numeric(10, 4), nullable=False)
    adjusted_close = db.Column(db.Numeric(10, 4), nullable=False)
    volume = db.Column(db.BigInteger, nullable=False)

    # Relationship to Stock model
    stock = relationship("Stock", back_populates="stockPrices")

    __table_args__ = (
        db.UniqueConstraint('stock_id', 'date', name='uq_stock_id_date'),
        db.Index('idx_stock_prices_date', 'date')
    )

    def __repr__(self):
        return f'<StockPrice(stock_id={self.stock_id}, date={self.date}, close={self.close})>'


@app.route('/', methods=['GET'])
async def hello():
    return make_response(jsonify({'message': 'Welcome to Stock Service'}), 200)

@app.route('/prices/<int:stock_id>', methods=['GET'])
async def getStockPrices(stock_id):
    try:
        if stock_id:
            prices = await db.session.query(StockPrice).filter_by(stock_id=stock_id).all()
            price_list = [{
                "stock_id": price.stock_id,
                "date": price.date,
                "open": price.open,
                "high": price.high,
                "low": price.low,
                "close": price.close,
                "adjusted_close": price.adjusted_close,
                "volume": price.volume
            } for price in prices]
            return make_response(jsonify(price_list), 200)
    except Exception as e:
        logging.error(f"Error getting stock prices: {e}")
        return make_response(jsonify({'error': 'Internal server error'}), 500)

@app.route('/prices', methods=['POST'])
async def pricesRoutes():
    if request.method == 'POST':
        try:
            data = await request.get_json()

            newPrice = StockPrice(
                stock_id=data.get('stock_id'),
                date=data.get('date'),
                open=data.get('open'),
                high=data.get('high'),
                low=data.get('low'),
                close=data.get('close'),
                adjusted_close=data.get('adjusted_close'),
                volume=data.get('volume')
            )
            # print(newPrice)
            db.session.add(newPrice)
            await db.session.commit()

            return make_response(jsonify({
                'stock_id': newPrice.stock_id
            }), 201)
        except Exception as e:
            logging.error(f"Error adding stock prices: {e}")
            return make_response(jsonify({'error': 'Internal server error'}), 500)

@app.route('/stocks', methods=['GET', 'POST'])
async def getStocks():
    if request.method == 'GET':
        # Query all stock objects
        try:
            stocks = await db.session.query(Stock).all()
            stock_list = [
                {
                    'stock_id': stock.stock_id,
                    'symbol': stock.symbol,
                    'name': stock.name,
                    'exchange': stock.exchange,
                    'sector': stock.sector,
                    'industry': stock.industry
                }
                for stock in stocks
            ]
            return jsonify(stock_list)
        except Exception as e:
            logging.error(f"Error retrieving stocks: {e}")
            return make_response(jsonify({'error': 'Internal server error'}), 500)
    elif request.method == 'POST':
        try:
            data = await request.get_json()
            symbol = data.get('symbol')
            name = data.get('name')
            exchange = data.get('exchange')
            sector = data.get('sector')
            industry = data.get('industry')

            newStock = Stock(
                symbol = symbol,
                name = name,
                exchange = exchange,
                sector = sector,
                industry = industry
            )

            db.session.add(newStock)
            await db.session.commit()
            
            return make_response(jsonify({
                'symbol': newStock.symbol,
                'name': newStock.name,
                'exchange': newStock.exchange,
                'sector': newStock.sector,
                'industry': newStock.industry
            }), 201)
        except Exception as e:
            logging.error(f'Error posting stock: {e}')
            return make_response(jsonify({'error': 'Internal server error'}), 500)


if __name__ == '__main__':
    print('Flask App starting on port 8000')
    app.run(debug=True, port=8000)